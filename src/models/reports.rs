use failure::Error;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentimentReport {
    positive: i64,
    negative: i64,
    neutral: i64,
    total: i64,
}

pub async fn get_sentiments_of_trend(
    db_pool: &DbPool,
    topic_id: &String,
) -> Result<Vec<SentimentReport>, Error> {
    let conn = db_pool.get().await?;
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

    conn.query(stmt, &[topic_id]).await?
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

pub async fn get_sentiments_of_trend_in_location(
    db_pool: &DbPool,
    topic_id: &String,
    woeid: &i32,
) -> Result<Vec<SentimentReport>, Error> {
    let conn = db_pool.get().await?;
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

    conn.query(stmt, &[topic_id, woeid]).await?
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
