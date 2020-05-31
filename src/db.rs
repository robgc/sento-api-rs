use bb8_postgres::tokio_postgres::NoTls;
use bb8_postgres::bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;

use crate::config::AppConfig;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn connect_to_db(app_config: &AppConfig) -> DbPool {
    let manager = PostgresConnectionManager::new(
        format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}",
            user = app_config.postgres.user,
            password = app_config.postgres.password,
            host = app_config.postgres.host,
            port = app_config.postgres.port,
            database = app_config.postgres.database,
        )
        .parse()
        .unwrap(),
        NoTls,
    );

    Pool::builder().build(manager).await.unwrap()
}
