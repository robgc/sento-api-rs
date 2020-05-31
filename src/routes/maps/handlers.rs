use actix_web::{web, Error as ActixWebError, HttpResponse};
use serde_json::json;

use crate::db::DbPool;
use crate::models::maps as Model;

pub async fn get_active_locations(
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, ActixWebError> {
    let query_result = Model::get_active_locations(&db_pool).await?;

    match query_result.first() {
        Some(x) => Ok(HttpResponse::Ok().json(x)),
        None => Ok(HttpResponse::Ok().json(json!([]))),
    }
}

pub async fn get_current_trends_for_location(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String,)>,
) -> Result<HttpResponse, ActixWebError> {
    let query_result = Model::get_current_trends_for_location(&db_pool, &path_params.0).await?;

    match query_result.first() {
        Some(x) => Ok(HttpResponse::Ok().json(x)),
        None => Ok(HttpResponse::Ok().json(json!([]))),
    }
}
