use actix_web::{web, Error as AWError, HttpResponse};
use futures::future::{Future, IntoFuture};
use serde::Deserialize;
use serde_json::json;

use crate::db::{execute_query, DbPool};
use crate::models::trends as Model;

#[derive(Debug, Deserialize)]
pub struct ListTrendsQParams {
    top: Option<bool>,
    woeid: Option<String>,
    name: Option<String>,
}

pub fn list_trends(
    db_pool: web::Data<DbPool>,
    query_params: web::Query<ListTrendsQParams>,
) -> Box<dyn Future<Item = HttpResponse, Error = AWError>> {
    let result: Box<dyn Future<Item = HttpResponse, Error = AWError>>;

    if query_params.top.unwrap_or(false) {
        result = Box::new(
            execute_query(&db_pool, Model::get_top_trends, None)
                .map_err(AWError::from)
                .map(|result| HttpResponse::Ok().json(result)),
        );
    } else if query_params.woeid.as_ref().unwrap_or(&"".to_owned()).len() > 0 {
        result = Box::new(
            execute_query(
                &db_pool,
                Model::get_current_trends_for_location,
                Some(query_params.woeid.as_ref().unwrap().to_owned()),
            )
            .map_err(AWError::from)
            .map(|result| HttpResponse::Ok().json(result)),
        );
    } else if query_params.name.as_ref().unwrap_or(&"".to_owned()).len() > 0 {
        result = Box::new(
            execute_query(
                &db_pool,
                Model::search_trends_by_name,
                Some(query_params.name.as_ref().unwrap().to_owned()),
            )
            .map_err(AWError::from)
            .map(|result| HttpResponse::Ok().json(result)),
        );
    } else {
        result = Box::new(HttpResponse::BadRequest().into_future());
    }

    result
}

pub fn get_trend_evolution_in_all_locations(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String,)>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    execute_query(
        &db_pool,
        Model::get_trend_evolution_in_all_locations,
        Some(path_params),
    )
    .map_err(AWError::from)
    .map(|result| HttpResponse::Ok().json(result))
}

pub fn get_trend_evolution_in_location(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String, i32)>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    execute_query(
        &db_pool,
        Model::get_trend_evolution_in_location,
        Some(path_params),
    )
    .map_err(AWError::from)
    .map(|result| {
        let alt_value = &json!([]);
        let response_value = result.first().unwrap_or(alt_value);
        HttpResponse::Ok().json(response_value)
    })
}
