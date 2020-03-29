use actix_web::web;

mod handlers;

pub fn routes_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("")
                .route(web::get().to_async(handlers::list_trends)),
        )
        .service(
            web::resource("/evolution/trends/{trend_id}")
                .route(web::get().to_async(handlers::get_trend_evolution_in_all_locations)),
        )
        .service(
            web::resource("/evolution/trends/{trend_id}/places/{woeid}")
                .route(web::get().to_async(handlers::get_trend_evolution_in_location)),
        );
}
