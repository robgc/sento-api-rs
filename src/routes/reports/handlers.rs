use actix_web::{web, Error as ActixWebError, HttpResponse};

use crate::db::{DbPool};
use crate::models::reports as Model;

pub async fn get_sentiment_report_by_trend(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String,)>,
) -> Result<HttpResponse, ActixWebError> {
    let query_result = Model::get_sentiments_of_trend(&db_pool, &path_params.0).await?;

    Ok(HttpResponse::Ok().json(query_result.first().unwrap()))
}

pub async fn get_sentiment_report_by_location_and_trend(
    db_pool: web::Data<DbPool>,
    path_params: web::Path<(String, i32)>,
) -> Result<HttpResponse, ActixWebError> {
    let query_result = Model::get_sentiments_of_trend_in_location(&db_pool, &path_params.0, &path_params.1).await?;

    Ok(HttpResponse::Ok().json(query_result.first().unwrap()))
}
