use failure::Error;

use actix_web::{web, Error as AWError};
use futures::future::Future;
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;

use crate::config::AppConfig;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;
pub type DbConnection = PooledConnection<PostgresConnectionManager<NoTls>>;

pub fn connect_to_db(app_config: &AppConfig) -> Result<DbPool, Error> {
    let manager = PostgresConnectionManager::new(
        format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}",
            user = app_config.postgres.user,
            password = app_config.postgres.password,
            host = app_config.postgres.host,
            port = app_config.postgres.port,
            database = app_config.postgres.database,
        )
        .parse()?,
        NoTls,
    );

    let pool = Pool::new(manager)?;

    Ok(pool)
}

pub fn execute_query<F, T, U>(
    pool: &DbPool,
    f: F,
    params: Option<U>,
) -> impl Future<Item = Vec<T>, Error = AWError>
where
    F: FnOnce(&mut DbConnection, Option<U>) -> Result<Vec<T>, Error>
        + Send
        + 'static,
    T: Send + 'static,
    U: Send + 'static,
{
    let p = pool.clone();

    web::block(move || f(&mut p.get()?, params)).from_err()
}
