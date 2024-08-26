use crate::middlewares::auth::{RequireAuth, StudentTokenValidator};
use crate::models::post_models::GradedStudentPostModel;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, ResponseError};
use application::database::groups::{get_group_by_id, get_group_id_of_student, get_students_from_group_for_evaluation};
use application::database::marks::create_mark;
use application::database::students_tokens::update_student_token;
use domain::models::groups::Group;
use domain::models::marks::NewMark;
use domain::models::students::Student;
use domain::models::students_tokens::UpdatedStudentToken;
use infrastructure::DBPool;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, DBError, InternalError, NotFoundError, ServerError, UnauthorizedError, UserError};
use uuid::Uuid;

/// Request group of current student to evaluate
///
/// This endpoint allows current student to get the group to evaluate.
#[utoipa::path(
    get,
    path = "/group-to-evaluate",
    tag = "Evaluation",
    context_path = "/marks",
    responses(
        (status = 200, description = "Respond with group students to evaluate", body = MinimalGroupStudents),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Group not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/group-to-evaluate")]
async fn get_group_to_evaluate_route(
    data: web::Data<AppState>,
    req: HttpRequest
) -> HttpResponse {
    // Get group from request
    let group = req.extensions().get::<Group>().cloned().unwrap();

    println!("Group: {:?}", group);

    let result = web::block(move || {
        let conn: DBPool = data.database_pool.clone().as_ref().clone();
        get_students_from_group_for_evaluation(&conn, group.id).map_err(APIError::from)
    }).await;

    match result {
        Ok(group) => match group {
            Ok(group) => HttpResponse::Ok().json(group),
            Err(err) => err.error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response()
    }
}

/// Evaluate group
///
/// This endpoint allows student to evaluate his group.
#[utoipa::path(
    post,
    path = "/evaluate/group/{group_id}",
    tag = "Evaluation",
    context_path = "/marks",
    params(
        ("group_id" = Uuid, description = "The group id to evaluate")
    ),
    request_body(
        content = [GradedStudentPostModel],
        description = "The graded students of a group",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The group was evaluated successfully", body = ()),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/evaluate/group/{group_id}")]
pub async fn evaluate_group_route(
    data: web::Data<AppState>,
    group_id: web::Path<Uuid>,
    body: web::Json<Vec<GradedStudentPostModel>>,
    req: HttpRequest
) -> HttpResponse {
    // Get student and group from request
    let student = req.extensions().get::<Student>().cloned().unwrap();
    let token_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let group_id = group_id.into_inner();

    let result = web::block(move || {
        let conn: DBPool = data.database_pool.clone().as_ref().clone();

        // Get group
        let group = get_group_by_id(&conn, group_id.clone())?;

        // Get student's group
        let student_group_id = get_group_id_of_student(&conn, student.id, group.project_id)?;
        if student_group_id.is_none() {
            return Err(APIError::UserError(UserError::NotFound(NotFoundError { resource: format!("Group for student {}", student.id) })));
        }

        // Check if student is in the group
        let group_id = group_id;
        if student_group_id.unwrap() != group_id {
            return Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)));
        }

        // Evaluate group
        register_group_grades(&conn, group.project_id, group_id, student.id, body.0).map_err(APIError::from)?;

        // Define token as used
        update_student_token(&conn, token_id, UpdatedStudentToken {
            used: Some(true),
        }).map_err(APIError::from)
    }).await;

    match result {
        Ok(_) => match result {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => err.error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response()
    }
}

fn register_group_grades(conn: &DBPool, project_id: Uuid, group_id: Uuid, student_id: Uuid, grades: Vec<GradedStudentPostModel>) -> Result<(), DBError> {
    for graded_student in grades {
        // Check if student is in the group
        let student_group_id = get_group_id_of_student(conn, graded_student.student_id, project_id)?;
        if student_group_id.is_none() || student_group_id.unwrap() != group_id {
            return Err(DBError::NotFound);
        }

        let new_mark = NewMark {
            project_id,
            group_id,
            noted_student_id: graded_student.student_id,
            grader_student_id: student_id,
            mark: graded_student.mark,
            max_mark: None,
            comment: graded_student.comment.clone(),
        };

        // Save mark
        create_mark(conn, new_mark)?;
    }

    Ok(())
}

pub fn marks_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/marks")
            .wrap(RequireAuth::new(StudentTokenValidator))
            .service(get_group_to_evaluate_route)
            .service(evaluate_group_route)
    );
}