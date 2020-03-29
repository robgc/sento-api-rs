use actix_web::web;
use failure::Error;
use serde::{Deserialize, Serialize};

use crate::db::DbConnection;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentimentReport {
    positive: i64,
    negative: i64,
    neutral: i64,
    total: i64,
}

pub fn get_sentiments_of_trend(
    conn: &mut DbConnection,
    params: Option<web::Path<(String,)>>,
) -> Result<Vec<SentimentReport>, Error> {
    let stmt = "
        SELECT
            COUNT(*) FILTER (WHERE sentiment = 1) as positive,
            COUNT(*) FILTER (WHERE sentiment = 0) as neutral,
            COUNT(*) FILTER (WHERE sentiment = -1) as negative,
            COUNT(*) AS total_count
        FROM
            data.statuses
        WHERE
            topic_id = $1;
    ";

    let param_values = params.unwrap();

    conn.query(stmt, &[&param_values.0])?
        .into_iter()
        .map(|row| {
            Ok(SentimentReport {
                positive: row.try_get(0)?,
                neutral: row.try_get(1)?,
                negative: row.try_get(2)?,
                total: row.try_get(3)?,
            })
        })
        .collect()
}

pub fn get_sentiments_of_trend_in_location(
    conn: &mut DbConnection,
    params: Option<web::Path<(String, i32)>>,
) -> Result<Vec<SentimentReport>, Error> {
    let stmt = "
        SELECT
            COUNT(*) FILTER (WHERE sentiment = 1) as positive,
            COUNT(*) FILTER (WHERE sentiment = 0) as neutral,
            COUNT(*) FILTER (WHERE sentiment = -1) as negative,
            COUNT(*) AS total_count
        FROM
            data.statuses
        WHERE
            topic_id = $1
            AND woeid = $2;
    ";

    let param_values = params.unwrap();

    conn.query(stmt, &[&param_values.0, &param_values.1])?
        .into_iter()
        .map(|row| {
            Ok(SentimentReport {
                positive: row.try_get(0)?,
                neutral: row.try_get(1)?,
                negative: row.try_get(2)?,
                total: row.try_get(3)?,
            })
        })
        .collect()
}
