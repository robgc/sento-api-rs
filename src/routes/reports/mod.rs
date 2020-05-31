use actix_web::web;

mod handlers;

pub fn routes_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/sentiment/trends/{trend_id}")
                .route(web::get().to(handlers::get_sentiment_report_by_trend)),
        )
        .service(
            web::resource("/sentiment/trends/{trend_id}/locations/{woeid}")
                .route(web::get().to(handlers::get_sentiment_report_by_location_and_trend)),
        );
}
