use crate::middlewares::auth::{RequireAuth, UserTokenValidator};
use crate::models::post_models::NewPromotionPostModel;
use crate::models::put_models::UpdatedPromotionPutModel;
use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse, ResponseError};
use application::database::promotions::{create_promotion, delete_promotion, get_all_promotions_from_teacher_id, get_promotion_by_id, get_promotions_by_matching_date_and_title, update_promotion, PromotionSearchParams};
use domain::models::promotions::{NewPromotion, UpdatedPromotion};
use domain::models::users::User;
use garde::Validate;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, ForbiddenError, InternalError, ServerError, UserError};
use uuid::Uuid;

/// Get all existing promotions from the current teacher
///
/// This endpoint returns all promotions from the current teacher in the database.
#[utoipa::path(
    get,
    path = "/",
    tag = "Promotions",
    context_path = "/promotions",
    responses(
        (status = 200, description = "All the returned promotions objects", body = [Promotion]),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/")]
pub async fn get_all_promotions_from_current_teacher_route(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let teacher = req.extensions().get::<User>().cloned().unwrap();
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        get_all_promotions_from_teacher_id(&conn, teacher.id)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(promotions) => HttpResponse::Ok().json(promotions),
            Err(err) => APIError::from(err).error_response()
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Get a promotion by id
///
/// This endpoint returns the promotion with the specified id.
#[utoipa::path(
    get,
    path = "/{id}",
    tag = "Promotions",
    context_path = "/promotions",
    params(
        ("id" = Uuid, description = "The promotion id to get")
    ),
    responses(
        (status = 200, description = "All the returned promotions objects", body = Promotion),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 403, description = "Forbidden", body = ForbiddenError, example = json!("Forbidden")),
        (status = 404, description = "Not Found", body = NotFoundError, example = json!("Database record")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/{id}")]
pub async fn get_promotion_by_id_route(data: web::Data<AppState>, req: HttpRequest, id: web::Path<Uuid>) -> HttpResponse {
    let teacher = req.extensions().get::<User>().cloned().unwrap();
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        let promotion = get_promotion_by_id(&conn, id.into_inner()).map_err(APIError::from)?;
        if promotion.teacher_id != teacher.id {
            return Err(APIError::UserError(UserError::Forbidden(ForbiddenError)));
        }

        Ok(promotion)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(promotion) => HttpResponse::Ok().json(promotion),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Get a promotion by title and date
///get_all_promotions_route
/// This endpoint returns the closest promotions to the specified title and date.
#[utoipa::path(
    get,
    path = "/search",
    tag = "Promotions",
    context_path = "/promotions",
    request_body(
        content = PromotionSearchParams,
        description = "The search parameters for the promotions",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "All the returned promotions objects", body = [Promotion]),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/search")]
pub async fn search_promotions_route(data: web::Data<AppState>, req: HttpRequest, params: web::Json<PromotionSearchParams>) -> HttpResponse {
    // Fix : Not working
    // Err : TypeError: Failed to execute 'fetch' on 'Window': Request with GET/HEAD method cannot have body.
    let teacher = req.extensions().get::<User>().cloned().unwrap();
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        let params = params.into_inner();
        get_promotions_by_matching_date_and_title(&conn, &params, teacher.id)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(promotions) => HttpResponse::Ok().json(promotions),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Create a new promotion
///
/// This endpoint creates a new promotion in the database.
#[utoipa::path(
    post,
    path = "/",
    tag = "Promotions",
    context_path = "/promotions",
    request_body(
        content = NewPromotionPostModel,
        description = "The new promotion object to create",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "Promotion created successfully", body = Uuid),
        (status = 400, description = "Bad Request", body = BadRequestError, examples(
            ("InvalidTitle" = (value = json!("Invalid title"))),
        )),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/")]
pub async fn create_promotion_route(data: web::Data<AppState>, req: HttpRequest, promotion: web::Json<NewPromotionPostModel>) -> HttpResponse {
    let teacher = req.extensions().get::<User>().cloned().unwrap();
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        promotion.validate().map_err(APIError::from)?;
        let new_promotion = NewPromotion {
            title: promotion.title.clone(),
            start_year: promotion.start_year,
            end_year: promotion.end_year,
            teacher_id: teacher.id,
        };
        create_promotion(&conn, new_promotion).map_err(APIError::from)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(id) => HttpResponse::Created().json(id),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Update a promotion
///
/// This endpoint updates a promotion in the database.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = "Promotions",
    context_path = "/promotions",
    params(
        ("id" = Uuid, description = "The promotion id to update")
    ),
    request_body(
        content = UpdatedPromotionPutModel,
        description = "The updated promotion object",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Promotion updated successfully"),
        (status = 400, description = "Bad Request", body = BadRequestError, examples(
            ("InvalidTitle" = (value = json!("Invalid title"))),
        )),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 403, description = "Forbidden", body = ForbiddenError, example = json!("Forbidden")),
        (status = 404, description = "Not Found", body = NotFoundError, example = json!("Database record")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[put("/{id}")]
pub async fn update_promotion_route(data: web::Data<AppState>, req: HttpRequest, id: web::Path<Uuid>, promotion: web::Json<UpdatedPromotionPutModel>) -> HttpResponse {
    let teacher = req.extensions().get::<User>().cloned().unwrap();
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        promotion.validate().map_err(APIError::from)?;

        let promotion_ = get_promotion_by_id(&conn, id.into_inner()).map_err(APIError::from)?;
        if promotion_.teacher_id != teacher.id {
            return Err(APIError::UserError(UserError::Forbidden(ForbiddenError)));
        }

        let updated_promotion = UpdatedPromotion {
            title: promotion.title.clone(),
            start_year: promotion.start_year,
            end_year: promotion.end_year,
        };
        update_promotion(&conn, promotion_.id, updated_promotion).map_err(APIError::from)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Delete a promotion
///
/// This endpoint deletes a promotion from the database.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "Promotions",
    context_path = "/promotions",
    params(
        ("id" = Uuid, description = "The promotion id to delete")
    ),
    responses(
        (status = 200, description = "Promotion deleted successfully"),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 403, description = "Forbidden", body = ForbiddenError, example = json!("Forbidden")),
        (status = 404, description = "Not Found", body = NotFoundError, example = json!("Database record")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[delete("/{id}")]
pub async fn delete_promotion_route(data: web::Data<AppState>, req: HttpRequest, id: web::Path<Uuid>) -> HttpResponse {
    let teacher = req.extensions().get::<User>().cloned().unwrap();
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();

        let promotion_ = get_promotion_by_id(&conn, id.into_inner()).map_err(APIError::from)?;
        if promotion_.teacher_id != teacher.id {
            return Err(APIError::UserError(UserError::Forbidden(ForbiddenError)));
        }

        delete_promotion(&conn, promotion_.id).map_err(APIError::from)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

pub fn promotions_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/promotions")
            .wrap(RequireAuth::new(UserTokenValidator))
            .service(get_all_promotions_from_current_teacher_route)
            .service(get_promotion_by_id_route)
            .service(search_promotions_route)
            .service(create_promotion_route)
            .service(update_promotion_route)
            .service(delete_promotion_route)
    );
}