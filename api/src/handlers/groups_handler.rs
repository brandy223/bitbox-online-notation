use crate::middlewares::auth::{RequireAuth, UserTokenValidator};
use crate::models::get_models::ProjectGroupsGetModel;
use crate::models::post_models::NewGroupPostModel;
use crate::models::put_models::UpdatedGroupPutModel;
use actix_web::{delete, get, post, put, web, HttpResponse, ResponseError};
use application::database::groups::{create_group, create_group_students, delete_group, get_group_student_mark_details, get_groups_and_students_from_project_id, get_students_without_group, remove_all_students_from_a_group, update_group};
use domain::models::groups::{NewGroup, NewGroupStudent, UpdatedGroup};
use garde::Validate;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, DBError, InternalError, ServerError};
use uuid::Uuid;

/// Get all the groups and the students from a project
///
/// This endpoint allows you to get all the groups and the corresponding students from a project in the database.
#[utoipa::path(
    get,
    path = "/project/{id}",
    tag = "Groups",
    context_path = "/groups",
    params(
        ("id" = Uuid, description = "The project id to get the groups from")
    ),
    responses(
        (status = 200, description = "All the returned groups", body = ProjectGroupsGetModel),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Not Found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/project/{id}")]
pub async fn get_groups_and_students_from_project_id_route(data: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || -> Result<ProjectGroupsGetModel, DBError> {
        let conn = data.database_pool.clone().as_ref().clone();
        let groups = get_groups_and_students_from_project_id(&conn, id.into_inner())?;
        Ok(ProjectGroupsGetModel {
            groups
        })
    }).await;

    match result {
        Ok(response) => match response {
            Ok(groups) => HttpResponse::Ok().json(groups),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Get the students which aren't in a group for a project
///
/// This endpoint returns all the students which aren't in a group for a chosen project
#[utoipa::path(
    get,
    path = "/project/{id}/students",
    tag = "Groups",
    context_path = "/groups",
    params(
        ("id" = Uuid, description = "The project id to get the students from")
    ),
    responses(
        (status = 200, description = "All the returned students", body = [Student]),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Not Found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/project/{id}/students")]
pub async fn get_students_without_group_route(data: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        let project_id = id.into_inner();
        get_students_without_group(&conn, project_id)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(students) => HttpResponse::Ok().json(students),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Returns the details of the 360 mark for a student in a group
///
/// This endpoint returns the details of the 360 mark for a student in a group containing all the marks and comments given by the other students of the group.
#[utoipa::path(
    get,
    path = "/{group_id}/student/{student_id}",
    tag = "Groups",
    context_path = "/groups",
    params(
        ("group_id" = Uuid, description = "The group id to get the student from"),
        ("student_id" = Uuid, description = "The student id to get the details from")
    ),
    responses(
        (status = 200, description = "The student details", body = StudentGroupMarkDetails),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Group or Student Not Found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/{group_id}/student/{student_id}")]
pub async fn get_group_student_mark_details_route(data: web::Data<AppState>, path: web::Path<(Uuid, Uuid)>) -> HttpResponse {
    let (group_id, student_id) = path.into_inner();
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        get_group_student_mark_details(&conn, group_id, student_id)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(group_student) => HttpResponse::Ok().json(group_student),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Create a group and add it to an existing project
///
/// This endpoint allows you to create a group and add it to an existing project in the database.
#[utoipa::path(
    post,
    path = "/project/{id}",
    tag = "Groups",
    context_path = "/groups",
    params(
        ("id" = Uuid, description = "The project id to add the group to")
    ),
    request_body(
        content = NewGroupPostModel,
        description = "The group to create"
    ),
    responses(
        (status = 201, description = "The group has been created", body = Uuid),
        (status = 400, description = "Bad Request", body = BadRequestError, examples(
            ("InvalidName" = (value = json!("Invalid name"))),
        )),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Project Not Found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/project/{id}")]
pub async fn create_group_route(data: web::Data<AppState>, id: web::Path<Uuid>, group: web::Json<NewGroupPostModel>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        group.validate().map_err(APIError::from)?;
        let new_group = NewGroup {
            name: group.name.clone(),
            project_id: id.into_inner(),
            max_mark: None,
        };
        create_group(&conn, new_group).map_err(APIError::from)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(group_id) => HttpResponse::Created().json(group_id),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Assigns students to a group
///
/// This endpoint allows you to assign students to a group in the database.
#[utoipa::path(
    post,
    path = "/{group_id}/students",
    tag = "Groups",
    context_path = "/groups",
    params(
        ("group_id" = Uuid, description = "The group id to assign the students to")
    ),
    request_body(
        content = [Uuid],
        description = "The students ids to assign",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The students have been assigned to the group"),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Group Not Found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/{group_id}/students")]
pub async fn assign_students_to_group_route(data: web::Data<AppState>, group_id: web::Path<Uuid>, students: web::Json<Vec<Uuid>>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        let group_id = group_id.into_inner();
        remove_all_students_from_a_group(&conn, group_id)?;

        let mut new_group_students: Vec<NewGroupStudent> = Vec::new();
        for student_id in students.into_inner() {
            new_group_students.push(NewGroupStudent {
                group_id,
                student_id,
            });
        }
        create_group_students(&conn, new_group_students)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Update a group
///
/// This endpoint allows you to update name or mark of a group in the database.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = "Groups",
    context_path = "/groups",
    params(
        ("id" = Uuid, description = "The group id to update")
    ),
    request_body(
        content = UpdatedGroupPutModel,
        description = "The group to update",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The group has been updated"),
        (status = 400, description = "Bad Request", body = BadRequestError, examples(
            ("InvalidName" = (value = json!("Invalid name"))),
        )),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Group Not Found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[put("/{id}")]
pub async fn update_group_route(data: web::Data<AppState>, id: web::Path<Uuid>, group: web::Json<UpdatedGroupPutModel>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        let group_id = id.into_inner();
        group.validate().map_err(APIError::from).map_err(APIError::from)?;
        let updated_group = UpdatedGroup {
            name: group.name.clone(),
            mark: group.mark,
            max_mark: None,
        };
        update_group(&conn, group_id, updated_group).map_err(APIError::from)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Delete a group
///
/// This endpoint allows you to delete a group in the database.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "Groups",
    context_path = "/groups",
    params(
        ("id" = Uuid, description = "The group id to delete")
    ),
    responses(
        (status = 200, description = "The group has been deleted"),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Group Not Found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[delete("/{id}")]
pub async fn delete_group_route(data: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        let group_id = id.into_inner();
        remove_all_students_from_a_group(&conn, group_id)?;
        delete_group(&conn, group_id)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

pub fn groups_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/groups")
            .wrap(RequireAuth::new(UserTokenValidator))
            .service(get_groups_and_students_from_project_id_route)
            .service(get_students_without_group_route)
            .service(get_group_student_mark_details_route)
            .service(create_group_route)
            .service(assign_students_to_group_route)
            .service(update_group_route)
            .service(delete_group_route)
    );
}