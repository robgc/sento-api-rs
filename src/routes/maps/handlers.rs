use actix_web::{web, Error as AWError, HttpResponse};
use futures::future::Future;
use serde_json::json;

use crate::db::{execute_query, DbPool};
use crate::models::maps as Model;

pub fn get_active_locations(
    db_pool: web::Data<DbPool>,
) -> Box<dyn Future<Item = HttpResponse, Error = AWError>> {
    Box::new(
        execute_query(&db_pool, Model::get_active_locations, None)
            .map_err(AWError::from)
            .map(|result| {
                let alt_value = &json!([]);
                let response_value = result.first().unwrap_or(alt_value);
                HttpResponse::Ok().json(response_value)
            }),
    )
}

pub fn get_current_trends_for_location(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String,)>,
) -> Box<dyn Future<Item = HttpResponse, Error = AWError>> {
    Box::new(
        execute_query(
            &db_pool,
            Model::get_current_trends_for_location,
            Some(path_params),
        )
        .map_err(AWError::from)
        .map(|result| {
            let alt_value = &json!([]);
            let response_value = result.first().unwrap_or(alt_value);
            HttpResponse::Ok().json(response_value)
        }),
    )
}
