table! {
    locations (id) {
        id -> Integer,
        timestamp -> Integer,
        latitude -> Float,
        longitude -> Float,
        altitude -> Nullable<Float>,
    }
}

table! {
    network_scans (id) {
        id -> Integer,
        network_id -> Integer,
        location_id -> Integer,
    }
}

table! {
    networks (id) {
        id -> Integer,
        bssid -> Text,
        ssid -> Nullable<Text>,
        mode -> Text,
        channel -> Integer,
        rate -> Integer,
        security -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    locations,
    network_scans,
    networks,
);
