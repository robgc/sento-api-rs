use actix_web::{web, Error as ActixWebError, HttpResponse};
use serde::Deserialize;

use crate::db::DbPool;
use crate::models::trends as Model;

#[derive(Debug, Deserialize)]
pub struct ListTrendsQParams {
    top: Option<bool>,
    woeid: Option<i32>,
    name: Option<String>,
}

pub async fn list_trends(
    db_pool: web::Data<DbPool>,
    query_params: web::Query<ListTrendsQParams>,
) -> Result<HttpResponse, ActixWebError> {
    let mut result: Result<HttpResponse, ActixWebError> = Ok(HttpResponse::BadRequest().finish());

    if query_params.top.unwrap_or(false) {
        result = Ok(HttpResponse::Ok().json(Model::get_top_trends(&db_pool).await?));
    }

    if let Some(woeid) = query_params.woeid {
        result = Ok(HttpResponse::Ok().json(Model::get_current_trends_for_location(&db_pool, &woeid).await?));
    }

    if let Some(name_query) = &query_params.name {
        if name_query.len() > 0 {
            result = Ok(HttpResponse::Ok().json(Model::search_trends_by_name(&db_pool, &name_query).await?));
        }
    }

    result
}

pub async fn get_trend_evolution_in_all_locations(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String,)>,
) -> Result<HttpResponse, ActixWebError> {
    let query_result = Model::get_trend_evolution_in_all_locations(&db_pool, &path_params.0).await?;

    Ok(HttpResponse::Ok().json(query_result))
}

pub async fn get_trend_evolution_in_location(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String, i32)>,
) -> Result<HttpResponse, ActixWebError> {
    let query_result = Model::get_trend_evolution_in_location(&db_pool, &path_params.0, &path_params.1).await?;

    Ok(HttpResponse::Ok().json(query_result))
}
