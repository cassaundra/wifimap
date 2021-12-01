use futures::future::{Fuse, FusedFuture, FutureExt};
use futures::stream::StreamExt;
use futures::{pin_mut, select};

use gpsd_proto::{Mode, Tpv};

use gpx::{Track, TrackSegment};
use log::*;

use tokio::time;
use tokio_stream::wrappers::IntervalStream;

use wifimap::gps::{GpsConnection, tpv_to_waypoint};
use wifimap::wifi::{WifiDevice, scan_devices};

use std::time::Duration;

use wifimap::db;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut db_con = db::establish_connection();

    let gps_con = GpsConnection::connect_localhost().await.unwrap().fuse();

    let mut interval_timer_stream =
        IntervalStream::new(time::interval(Duration::from_millis(10))).fuse();

    let wifi_fut = Fuse::terminated();

    pin_mut!(wifi_fut, gps_con);

    let mut last_gps = None;
    let mut last_wifi = None;

    let mut gps_wifi_pairs = vec![];

    let mut track_segment = TrackSegment::new();

    while track_segment.points.len() < 64 {
        if let (Some(gps_data), Some(wifi_data)) = (&last_gps, &last_wifi) {
            debug!("Got both.");

            if let Tpv { mode: Mode::Fix3d, lat: Some(lat), lon: Some(lon), alt: Some(alt), .. } = gps_data {
                let location_id = db::insert_location(&mut db_con, 4, *lat as f32, *lon as f32, Some(*alt as f32));

            }

            // db::create_gps_data(&mut db_con, 4, gps_data.lat.unwrap(), gps_data.lon.unwrap());
            // db::create_gps_data(&mut db_con, gps_data, 0.0, 0.0);

            // if let Some(Tpv { mode: Mode::Fix3d, lat: Some(lat), lon: Some(lon), alt: Some(alt), .. }) = gps_data {
            //     gpx.waypoints.push(gpx::Waypoint {
            //         elevation: Some(alt),
            //         ..Default::default(),
            //     }));
            // }

            gps_wifi_pairs.push((last_gps, last_wifi));

            last_gps = None;
            last_wifi = None;
        }

        select! {
            _interval = interval_timer_stream.select_next_some() => {
                if wifi_fut.is_terminated() {
                    wifi_fut.set(scan_devices().fuse());
                }
            },
            gps_data = gps_con.select_next_some() => {
                if let Some(Tpv { mode: Mode::Fix3d, lat: Some(lat), lon: Some(lon), alt: Some(alt), .. }) = gps_data {
                    debug!("data: {} {} {}", lat, lon, alt);

                    let data = gps_data.unwrap();
                    track_segment.points.push(tpv_to_waypoint(&data).unwrap());
                    last_gps = Some(data);
                }
            },
            wifi_devices = wifi_fut => {
                if let Ok(wifi_devices) = wifi_devices {
                    debug!("Got {} Wi-Fi devices", wifi_devices.len());
                    last_wifi = Some(wifi_devices);
                }
            },
            complete => break,
        }
    }

    let mut gpx = gpx::Gpx::default();
    gpx.version = gpx::GpxVersion::Gpx11;

    let mut track = Track::new();
    track.segments.push(track_segment);

    gpx.tracks.push(track);

    gpx::write(&gpx, std::io::stdout()).unwrap();

    let mut writer = csv::WriterBuilder::new()
        .delimiter('\t' as u8)
        .from_writer(std::io::stdout());
    for record in gps_wifi_pairs {
        // writer.write_record(record.1);
        writer.serialize(record.1).unwrap();
    }
}

// fn clean_gps(devices: &[WifiDevice]) -> impl Iterator<Item = WifiDevice> {
//     devices.iter()
//         .filter(|dev| true)
// }
