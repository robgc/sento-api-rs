use actix_web::web;
use failure::Error;
use serde_json;

use crate::db::DbConnection;

pub fn get_active_locations(
    conn: &mut DbConnection,
    _params: Option<()>,
) -> Result<Vec<serde_json::Value>, Error> {
    let stmt = "
        WITH active_locations as (
            SELECT DISTINCT
                woeid
            FROM
                data.rankings
            WHERE
                ranking_ts >= (
                    (SELECT MAX(ranking_ts) FROM data.rankings)
                    - interval '12 hours'
                )
        ), active_locations_as_geojson as (
            SELECT
                jsonb_build_object(
                    'type', 'Feature',
                    'id', locs.id,
                    'geometry', ST_AsGeoJSON(locs.the_geom_point)::jsonb,
                    'properties', json_build_object(
                    'name', locs.name,
                    'osm_name', locs.osm_name
                    )
                ) AS feature
            FROM
                active_locations al
            JOIN data.locations locs ON al.woeid = locs.id
        )

        SELECT
            jsonb_build_object(
                'type', 'FeatureCollection',
                'features', jsonb_agg(feature)
            )
        FROM
            active_locations_as_geojson;
    ";

    conn.query(stmt, &[])?
        .into_iter()
        .map(|row| { Ok(row.try_get(0)?) })
        .collect()
}

pub fn get_current_trends_for_location(
    conn: &mut DbConnection,
    params: Option<web::Path<(String,)>>,
) -> Result<Vec<serde_json::Value>, Error> {
    let stmt = "
        WITH filtered_locations as (
            SELECT DISTINCT
                woeid
            FROM
                data.rankings
            WHERE
                topic_id = $1
            ), filtered_locations_as_geojson as (
            SELECT
                jsonb_build_object(
                    'type', 'Feature',
                    'id', locs.id,
                    'geometry', ST_AsGeoJSON(locs.the_geom_point)::jsonb,
                    'properties', json_build_object(
                        'name', locs.name,
                        'osm_name', locs.osm_name
                    )
                ) AS feature
            FROM
                filtered_locations fl
                JOIN data.locations locs ON fl.woeid = locs.id
            )

            SELECT
                jsonb_build_object(
                    'type', 'FeatureCollection',
                    'features', jsonb_agg(feature)
                )
            FROM
                filtered_locations_as_geojson;
    ";

    let param_values = params.unwrap();

    conn.query(stmt, &[&param_values.0])?
        .into_iter()
        .map(|row| { Ok(row.try_get(0)?) })
        .collect()
}
