use actix_web::web;
use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::db::DbConnection;

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

pub fn get_top_trends(
    conn: &mut DbConnection,
    _params: Option<()>,
) -> Result<Vec<TopTrends>, Error> {
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

    conn.query(stmt, &[])?
        .into_iter()
        .map(|row| {
            Ok(TopTrends {
                topic_id: row.try_get(0)?,
                ranking_no: row.try_get(1)?,
            })
        })
        .collect()
}

pub fn get_current_trends_for_location(
    conn: &mut DbConnection,
    location_woeid: Option<String>,
) -> Result<Vec<CurrentTrendsForLocation>, Error> {
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

    conn.query(stmt, &[&location_woeid.unwrap().parse::<i32>()?])?
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

pub fn get_trend_evolution_in_all_locations(
    conn: &mut DbConnection,
    params: Option<web::Path<(String,)>>,
) -> Result<Vec<TrendEvolutionInAllLocations>, Error> {
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

    let param_values = params.unwrap();

    conn.query(stmt, &[&param_values.0])?
        .into_iter()
        .map(|row| {
            Ok(TrendEvolutionInAllLocations {
                location: row.try_get(0)?,
                evolution: row.try_get(1)?,
            })
        })
        .collect()
}

pub fn get_trend_evolution_in_location(
    conn: &mut DbConnection,
    params: Option<web::Path<(String, i32)>>,
) -> Result<Vec<serde_json::Value>, Error> {
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

    let param_values = params.unwrap();

    conn.query(stmt, &[&param_values.0, &param_values.1])?
        .into_iter()
        .map(|row| { Ok(row.try_get(0)?) })
        .collect()
}

pub fn search_trends_by_name(
    conn: &mut DbConnection,
    name: Option<String>,
) -> Result<Vec<SearchResult>, Error> {
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

    conn.query(stmt, &[&name.unwrap()])?
        .into_iter()
        .map(|row| {
            Ok(SearchResult {
                id: row.try_get(0)?,
                distance: row.try_get(1)?,
            })
        })
        .collect()
}
