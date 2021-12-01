use super::schema::*;

#[derive(Queryable)]
pub struct Network {
    pub id: i32,
    pub bssid: String,
    pub ssid: String,
    pub mode: String,
    pub channel: i32,
    pub rate: i32,
    pub security: String,
}

#[derive(Insertable)]
#[table_name = "networks"]
pub struct NewNetwork {
    pub bssid: String,
    pub ssid: String,
    pub mode: String,
    pub channel: i32,
    pub rate: i32,
    pub security: String,
}

#[derive(Queryable)]
pub struct Location {
    pub id: i32,
    pub timestamp: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: Option<f32>,
}

#[derive(Insertable)]
#[table_name = "locations"]
pub struct NewLocation {
    pub timestamp: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: Option<f32>,
}

#[derive(Queryable)]
pub struct NetworkScan {
    pub id: i32,
    pub network_id: i32,
    pub location_id: i32,
}

#[derive(Insertable)]
#[table_name = "network_scans"]
pub struct NewNetworkScan {
    pub network_id: i32,
    pub location_id: i32
}
