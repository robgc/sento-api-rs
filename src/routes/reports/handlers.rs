use actix_web::{web, Error as AWError, HttpResponse};
use futures::future::Future;

use crate::db::{execute_query, DbPool};
use crate::models::reports as Model;

pub fn get_sentiment_report_by_trend(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String,)>,
) -> Box<dyn Future<Item = HttpResponse, Error = AWError>> {
    Box::new(
        execute_query(
            &db_pool,
            Model::get_sentiments_of_trend,
            Some(path_params),
        )
        .map_err(AWError::from)
        .map(|result| {
            let response_value = result.first().unwrap();
            HttpResponse::Ok().json(response_value)
        })
    )
}

pub fn get_sentiment_report_by_location_and_trend(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String, i32)>,
) -> Box<dyn Future<Item = HttpResponse, Error = AWError>> {
    Box::new(
        execute_query(
            &db_pool,
            Model::get_sentiments_of_trend_in_location,
            Some(path_params),
        )
        .map_err(AWError::from)
        .map(|result| {
            let response_value = result.first().unwrap();
            HttpResponse::Ok().json(response_value)
        })
    )
}
