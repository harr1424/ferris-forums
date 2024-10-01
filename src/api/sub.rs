use crate::model::sub::Sub;
use crate::repo::sub as sub_repo;
use actix_web::{delete, get, patch, post, put, web::Data, web::Json, web::Path, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

#[post("/subs")]
pub async fn create_sub(
    pool: Data<PgPool>,
    body: Json<Sub>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_sub = Sub {
        name: body.name.clone(),
        description: body.description.clone(),
        created_at: Utc::now(),
    };

    let sub_id = sub_repo::create_sub(&pool, &new_sub)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(sub_id.to_string()))
}

#[put("/subs/{sub_name}")]
pub async fn subscribe_user_to_sub(
    pool: Data<PgPool>,
    path: Path<String>,
    body: String,
) -> Result<HttpResponse, actix_web::Error> {
    let sub_name = path.into_inner();
    let user_id = match body.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid user ID")),
    };
    sub_repo::subscribe_user_to_sub(&pool, user_id, &sub_name)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(format!("{} has been subscribed to {}", body, sub_name)))
}

#[get("/subs")]
pub async fn get_all_subs(pool: Data<PgPool>) -> Result<Json<Vec<Sub>>, actix_web::Error> {
    let subs = sub_repo::get_all_subs(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(Json(subs))
}

#[get("/subs/for_user/{user_id}")]
pub async fn get_subs_by_user_id(
    pool: Data<PgPool>,
    path: Path<i32>,
) -> Result<Json<Vec<Sub>>, actix_web::Error> {
    let user_id = path.into_inner();

    let subs = sub_repo::get_subs_by_user_id(&pool, user_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(Json(subs))
}

#[get("/subs/{name}")]
pub async fn get_sub_by_name(
    pool: Data<PgPool>,
    path: Path<String>,
) -> Result<Json<Sub>, actix_web::Error> {
    let name = path.into_inner();

    let sub = sub_repo::get_sub_by_name(&pool, &name)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(Json(sub))
}

#[patch("/subs")]
pub async fn update_sub(
    pool: Data<PgPool>,
    body: Json<Sub>,
) -> Result<HttpResponse, actix_web::Error> {
    let sub = body.into_inner();
    let (name, description) = sub_repo::update_sub(&pool, &sub)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(format!("{} -> {}", name, description)))
}

#[delete("/subs/{name}")]
pub async fn delete_sub(
    pool: Data<PgPool>,
    path: Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let name = path.into_inner();

    sub_repo::delete_sub(&pool, name.clone())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(format!("{} was deleted", name)))
}
