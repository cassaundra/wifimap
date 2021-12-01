pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// TODO reduce boilerplate

pub fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_network(
    conn: &mut SqliteConnection, bssid: String, ssid: String, mode: String, channel: i32,
    rate: i32, security: String,
) -> crate::Result<usize> {
    use models::NewNetwork;
    use schema::networks;

    let new_network = NewNetwork {
        bssid,
        ssid,
        mode,
        channel,
        rate,
        security,
    };

    let id = diesel::insert_into(networks::table)
        .values(&new_network)
        .execute(conn)?;
    Ok(id)
}

pub fn insert_location(
    conn: &mut SqliteConnection, timestamp: i32, latitude: f32, longitude: f32, altitude: Option<f32>,
) -> crate::Result<usize> {
    use models::NewLocation;
    use schema::locations;

    let new_location = NewLocation {
        timestamp,
        latitude,
        longitude,
        altitude,
    };

    let id = diesel::insert_into(locations::table)
        .values(&new_location)
        .execute(conn)?;
    Ok(id)
}

pub fn insert_network_scan(
    conn: &mut SqliteConnection, network_id: i32, location_id: i32) -> crate::Result<usize> {
    use models::NewNetworkScan;
    use schema::network_scans;

    let new_network_scan = NewNetworkScan {
        network_id,
        location_id,
    };

    let id = diesel::insert_into(network_scans::table)
        .values(&new_network_scan)
        .execute(conn)?;
    Ok(id)
}
