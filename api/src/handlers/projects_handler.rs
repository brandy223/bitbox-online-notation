use crate::middlewares::auth::{RequireAuth, UserTokenValidator};
use crate::models::post_models::NewProjectPostModel;
use crate::models::put_models::UpdatedProjectPutModel;
use actix_web::{delete, get, post, put, web, HttpResponse, ResponseError};
use application::database::projects::{create_project, delete_project, get_project_by_id, get_projects_from_promotion_id, update_project};
use chrono::Utc;
use domain::models::projects::{NewProject, ProjectState, UpdatedProject};
use garde::Validate;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, InternalError, ServerError};
use uuid::Uuid;

/// Get all projects from a promotion
///
/// This endpoint returns all projects from a promotion in the database.
#[utoipa::path(
    get,
    path = "/promotion/{promotion_id}",
    tag = "Projects",
    context_path = "/projects",
    params(
        ("promotion_id" = Uuid, description = "The promotion id to get the projects from")
    ),
    responses(
        (status = 200, description = "All the returned projects objects", body = [Project]),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/promotion/{promotion_id}")]
pub async fn get_projects_from_promotion_route(data: web::Data<AppState>, promotion_id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        get_projects_from_promotion_id(&conn, promotion_id.into_inner())
    }).await;

    match result {
        Ok(response) => match response {
            Ok(projects) => HttpResponse::Ok().json(projects),
            Err(err) => APIError::from(err).error_response()
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Get a project
///
/// This endpoint returns a project from the database.
#[utoipa::path(
    get,
    path = "/{id}",
    tag = "Projects",
    context_path = "/projects",
    params(
        ("id" = Uuid, description = "The project id to get")
    ),
    responses(
        (status = 200, description = "The returned project object", body = Project),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Project not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/{id}")]
pub async fn get_project_route(data: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        get_project_by_id(&conn, id.into_inner())
    }).await;

    match result {
        Ok(response) => match response {
            Ok(project) => HttpResponse::Ok().json(project),
            Err(err) => APIError::from(err).error_response()
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Create a new project for a promotion
///
/// This endpoint creates a new project for a promotion in the database.
#[utoipa::path(
    post,
    path = "/promotion/{promotion_id}",
    tag = "Projects",
    context_path = "/projects",
    params(
        ("id" = Uuid, description = "The promotion id to create a project for")
    ),
    request_body(
        content = NewProjectPostModel,
        description = "The project object to create",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "The project was created successfully", body = Uuid),
        (status = 400, description = "Bad Request", body = BadRequestError, examples(
            ("InvalidName" = (value = json!("Invalid name"))),
        )),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Project not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/promotion/{promotion_id}")]
pub async fn create_project_route(data: web::Data<AppState>, promotion_id: web::Path<Uuid>, project: web::Json<NewProjectPostModel>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        project.validate().map_err(APIError::from)?;

        // Check project date
        let mut state: ProjectState = ProjectState::NotStarted;
        if project.start_date.is_some() && project.start_date.unwrap() < Utc::now().naive_utc() {
            state = ProjectState::InProgress;
        }
        if project.end_date < Utc::now().naive_utc() {
            state = ProjectState::Finished;
        }

        let new_project = NewProject {
            name: project.name.clone(),
            description: project.description.clone(),
            start_date: project.start_date,
            end_date: project.end_date,
            notation_period_duration: project.notation_period_duration,
            promotion_id: promotion_id.into_inner(),
            state: Some(state),
        };
        create_project(&conn, new_project).map_err(APIError::from)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(project_id) => HttpResponse::Created().json(project_id),
            Err(err) => APIError::from(err).error_response()
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Update a project
///
/// This endpoint updates a project in the database.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = "Projects",
    context_path = "/projects",
    params(
        ("id" = Uuid, description = "The project id to update")
    ),
    request_body(
        content = UpdatedProjectPutModel,
        description = "The project object to update",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The project was updated successfully", body = ()),
        (status = 400, description = "Bad Request", body = BadRequestError, examples(
            ("InvalidName" = (value = json!("Invalid name"))),
        )),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Project not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[put("/{id}")]
pub async fn update_project_route(data: web::Data<AppState>, id: web::Path<Uuid>, project: web::Json<UpdatedProjectPutModel>) -> HttpResponse {
    // TODO : Check if project has started or not
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        project.validate().map_err(APIError::from)?;
        let updated_project = UpdatedProject {
            name: project.name.clone(),
            description: project.description.clone(),
            start_date: project.start_date,
            end_date: project.end_date,
            notation_period_duration: project.notation_period_duration,
            state: None,
        };
        update_project(&conn, id.into_inner(), updated_project).map_err(APIError::from)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response()
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Delete a project
///
/// This endpoint deletes a project in the database.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "Projects",
    context_path = "/projects",
    params(
        ("id" = Uuid, description = "The project id to delete")
    ),
    responses(
        (status = 200, description = "The project was deleted successfully", body = ()),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Project not found", body = NotFoundError, example = json!("NotFoundError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[delete("/{id}")]
pub async fn delete_project_route(data: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        delete_project(&conn, id.into_inner())
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response()
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

pub fn projects_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .wrap(RequireAuth::new(UserTokenValidator))
            .service(get_projects_from_promotion_route)
            .service(get_project_route)
            .service(create_project_route)
            .service(update_project_route)
            .service(delete_project_route)
    );
}