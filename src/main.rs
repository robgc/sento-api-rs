use sento_api::run_server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    run_server().await
}
