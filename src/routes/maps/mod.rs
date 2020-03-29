use actix_web::web;

mod handlers;

pub fn routes_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/active").route(web::get().to_async(handlers::get_active_locations)),
        )
        .service(
            web::resource("/trends/{trend_id}")
                .route(web::get().to_async(handlers::get_current_trends_for_location)),
        );
}
