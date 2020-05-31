use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::db::DbPool;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopTrends {
    topic_id: String,
    ranking_no: i16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentTrendsForLocation {
    ranking_ts: String,
    ranking_no: i16,
    tweet_volume: Option<i32>,
    topic_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendsEvolutionInLocation {
    timestamp: String,
    positions: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendEvolutionInAllLocations {
    location: String,
    evolution: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    id: String,
    distance: f32,
}

pub async fn get_top_trends(
    db_pool: &DbPool,
) -> Result<Vec<TopTrends>, Error> {
    let conn = db_pool.get().await?;
    let stmt = "
    SELECT DISTINCT
        topic_id,
        ranking_no
    FROM
        data.rankings rankings
    WHERE
        ranking_no BETWEEN 1 AND 10
        AND ranking_ts >= ((SELECT MAX(ranking_ts) FROM data.rankings) - interval '12 hours')
    ORDER BY
        ranking_no ASC
    LIMIT 15;
    ";

    conn.query(stmt, &[]).await?
        .into_iter()
        .map(|row| {
            Ok(TopTrends {
                topic_id: row.try_get(0)?,
                ranking_no: row.try_get(1)?,
            })
        })
        .collect()
}

pub async fn get_current_trends_for_location(
    db_pool: &DbPool,
    location_woeid: &i32
) -> Result<Vec<CurrentTrendsForLocation>, Error> {
    let conn = db_pool.get().await?;
    let stmt = "
        SELECT
            to_char(ranking_ts at time zone 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') as ranking_ts,
            ranking_no,
            tweet_volume,
            topic_id
        FROM (
            SELECT *
            FROM
                data.rankings
            WHERE
                woeid = $1
                AND ranking_ts >= ((SELECT MAX(ranking_ts) FROM data.rankings) - interval '12 hours')
            ORDER BY
                ranking_ts desc
            LIMIT 50
        ) trends
        ORDER BY
            ranking_no asc;
    ";

    conn.query(stmt, &[location_woeid]).await?
        .into_iter()
        .map(|row| {
            Ok(CurrentTrendsForLocation {
                ranking_ts: row.try_get(0)?,
                ranking_no: row.try_get(1)?,
                tweet_volume: row.try_get(2)?,
                topic_id: row.try_get(3)?,
            })
        })
        .collect()
}

pub async fn get_trend_evolution_in_all_locations(
    db_pool: &DbPool,
    topic_id: &String,
) -> Result<Vec<TrendEvolutionInAllLocations>, Error> {
    let conn = db_pool.get().await?;
    let stmt = "
        WITH evolution_by_location AS (
            SELECT
            woeid,
            json_agg(
                json_build_object(
                'timestamp', to_char(ranking_ts, 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"'),
                'position', ranking_no
                )
                ORDER BY ranking_ts ASC
            ) AS \"location_evolution\"
            FROM
            data.rankings rankings
            WHERE
            topic_id = $1
            GROUP BY
            woeid
        )

        SELECT
            locations.name AS \"location\",
            evolution.location_evolution AS \"evolution\"
        FROM
            evolution_by_location evolution
            JOIN data.locations locations ON evolution.woeid = locations.id;
    ";

    conn.query(stmt, &[topic_id]).await?
        .into_iter()
        .map(|row| {
            Ok(TrendEvolutionInAllLocations {
                location: row.try_get(0)?,
                evolution: row.try_get(1)?,
            })
        })
        .collect()
}

pub async fn get_trend_evolution_in_location(
    db_pool: &DbPool,
    topic_id: &String,
    woeid: &i32,
) -> Result<Vec<serde_json::Value>, Error> {
    let conn = db_pool.get().await?;
    let stmt = "
        SELECT
            json_agg(
                json_build_object(
                    'timestamp', to_char(ranking_ts, 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"'),
                    'number', ranking_no
                )
                ORDER BY ranking_ts ASC
            )
        FROM
            data.rankings
        WHERE
            topic_id = $1
            AND woeid = $2
        GROUP BY
            topic_id,
            woeid;
    ";

    conn.query(stmt, &[topic_id, woeid]).await?
        .into_iter()
        .map(|row| { Ok(row.try_get(0)?) })
        .collect()
}

pub async fn search_trends_by_name(
    db_pool: &DbPool,
    query_word: &String,
) -> Result<Vec<SearchResult>, Error> {
    let conn = db_pool.get().await?;
    let stmt = "
        SELECT
            id AS trend,
            id <-> $1 AS distance
        FROM
            data.topics
        ORDER BY
            distance ASC
        LIMIT 20
    ";

    conn.query(stmt, &[query_word]).await?
        .into_iter()
        .map(|row| {
            Ok(SearchResult {
                id: row.try_get(0)?,
                distance: row.try_get(1)?,
            })
        })
        .collect()
}
