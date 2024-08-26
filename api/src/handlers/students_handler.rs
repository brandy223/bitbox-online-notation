use crate::middlewares::auth::{RequireAuth, UserTokenValidator};
use crate::models::post_models::NewStudentPostModel;
use crate::models::put_models::UpdatedStudentPutModel;
use actix_web::{delete, get, post, put, web, HttpResponse, ResponseError};
use application::database::groups::remove_students_from_groups;
use application::database::marks::delete_all_marks_from_student;
use application::database::students::{create_promotion_students, create_student, delete_student, get_student_by_id, get_students_from_promotion_id, remove_student_from_all_promotions, update_student};
use application::database::students_tokens::delete_all_tokens_from_student;
use domain::models::students::{NewPromotionStudent, NewStudent, UpdatedStudent};
use garde::Validate;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, InternalError, ServerError};
use uuid::Uuid;

/// Get a student
///
/// This endpoint allows you to get a student in the database.
#[utoipa::path(
    get,
    path = "/{id}",
    tag = "Students",
    context_path = "/students",
    params(
        ("id" = Uuid, description = "The student id to get")
    ),
    responses(
        (status = 200, description = "The returned student", body = Student),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Student not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/{id}")]
pub async fn get_student_route(data: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        get_student_by_id(&conn, id.into_inner())
    }).await;

    match result {
        Ok(response) => match response {
            Ok(student) => HttpResponse::Ok().json(student),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Get all students from a promotion
///
/// This endpoint allows you to get all students from a promotion in the database.
#[utoipa::path(
    get,
    path = "/promotion/{id}",
    tag = "Students",
    context_path = "/students",
    params(
        ("id" = Uuid, description = "The promotion id to get the students from")
    ),
    responses(
        (status = 200, description = "All the returned students", body = Promotion),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/promotion/{id}")]
pub async fn get_students_from_promotion_id_route(data: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        get_students_from_promotion_id(&conn, id.into_inner())
    }).await;

    match result {
        Ok(response) => match response {
            Ok(students) => HttpResponse::Ok().json(students),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Create a student and add it to an existing promotion
///
/// This endpoint allows you to create a student and add it to an existing promotion in the database.
#[utoipa::path(
    post,
    path = "/promotion/{id}",
    tag = "Students",
    context_path = "/students",
    params(
        ("id" = Uuid, description = "The promotion id to add the student to")
    ),
    request_body(
        content = [NewStudentPostModel],
        description = "The students to add",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "The student(s) has been created and added to the promotion", body = Uuid),
        (status = 400, description = "Bad Request", body = BadRequestError, examples(
            ("InvalidEmail" = (value = json!("Invalid email"))),
            ("InvalidName" = (value = json!("Invalid name"))),
            ("InvalidSurname" = (value = json!("Invalid surname"))),
        )),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Promotion not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/promotion/{id}")]
pub async fn create_student_for_promotion_route(data: web::Data<AppState>, id: web::Path<Uuid>, student: web::Json<NewStudentPostModel>) -> HttpResponse {
    let result = web::block(move || -> Result<Uuid, APIError> {
        println!("Creating student for promotion");
        let conn = data.database_pool.clone().as_ref().clone();
        let promotion_id = id.into_inner();
        let student = student.into_inner();
        student.validate().map_err(APIError::from)?;
        let new_student = NewStudent {
            name: student.name,
            surname: student.surname,
            email: student.email,
        };
        let student_id = create_student(&conn, new_student).map_err(APIError::from)?;
        create_promotion_students(&conn, vec![NewPromotionStudent {
            promotion_id,
            student_id,
        }]).map_err(APIError::from)?;
        Ok(student_id)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(student_id) => HttpResponse::Created().json(student_id),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Update a student
///
/// This endpoint allows you to update a student in the database.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = "Students",
    context_path = "/students",
    params(
        ("id" = Uuid, description = "The student id to update")
    ),
    request_body(
        content = UpdatedStudentPutModel,
        description = "The student to update",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The student has been updated"),
        (status = 400, description = "Bad Request", body = BadRequestError, examples(
            ("InvalidEmail" = (value = json!("Invalid email"))),
            ("InvalidName" = (value = json!("Invalid name"))),
            ("InvalidSurname" = (value = json!("Invalid surname"))),
        )),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Student not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[put("/{id}")]
pub async fn update_student_route(data: web::Data<AppState>, id: web::Path<Uuid>, student_: web::Json<UpdatedStudentPutModel>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        let student_id = id.into_inner();
        let student = student_.into_inner();
        student.validate().map_err(APIError::from)?;
        let updated_student = UpdatedStudent {
            name: student.name,
            surname: student.surname,
            email: student.email,
        };
        update_student(&conn, student_id, updated_student).map_err(APIError::from)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Delete a student
///
/// This endpoint allows you to delete a student in the database.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "Students",
    context_path = "/students",
    params(
        ("id" = Uuid, description = "The student id to delete")
    ),
    responses(
        (status = 200, description = "The student has been deleted"),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Student not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[delete("/{id}")]
pub async fn delete_student_route(data: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        let student_id = id.into_inner();
        remove_students_from_groups(&conn, student_id).map_err(APIError::from)?;
        delete_all_tokens_from_student(&conn, student_id).map_err(APIError::from)?;
        delete_all_marks_from_student(&conn, student_id).map_err(APIError::from)?;
        remove_student_from_all_promotions(&conn, student_id).map_err(APIError::from)?;
        delete_student(&conn, student_id).map_err(APIError::from)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

pub fn students_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/students")
            .wrap(RequireAuth::new(UserTokenValidator))
            .service(get_student_route)
            .service(get_students_from_promotion_id_route)
            .service(create_student_for_promotion_route)
            .service(update_student_route)
            .service(delete_student_route)
    );
}