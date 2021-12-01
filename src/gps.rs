use std::{
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};

use chrono::DateTime;
use futures::{prelude::*, sink::SinkExt, stream::StreamExt};

use geo_types::Point;
use gpsd_proto::{Mode, Tpv, UnifiedResponse};

use gpx::Waypoint;

use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

pub struct GpsConnection {
    framed: Framed<TcpStream, LinesCodec>,
}

impl GpsConnection {
    pub async fn connect(addr: &SocketAddr) -> io::Result<Self> {
        // connect to local gpsd
        let sock = TcpStream::addconnect(addr).await?;
        let mut framed = Framed::new(sock, LinesCodec::new());

        // enable watch
        framed.send(gpsd_proto::ENABLE_WATCH_CMD.to_string()).await.unwrap();

        Ok(GpsConnection {
            framed,
        })
    }

    pub async fn connect_localhost() -> io::Result<Self> {
        GpsConnection::connect(&"127.0.0.1:2947".parse().unwrap()).await
    }
}

impl Stream for GpsConnection {
    type Item = Option<Tpv>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // TODO clean up
        match self.get_mut().framed.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(line))) => {
                match serde_json::from_str(&line) {
                    Ok(rd) => match rd {
                        UnifiedResponse::Version(v) => {
                            if v.proto_major < gpsd_proto::PROTO_MAJOR_MIN {
                                panic!("GPSD major version mismatch");
                            }

                            info!("GPSD version {} connected", v.rev);
                        }
                        UnifiedResponse::Tpv(tpv) => match tpv.mode {
                            Mode::Fix2d | Mode::Fix3d => return Poll::Ready(Some(Some(tpv))),
                            Mode::NoFix => {}
                        },
                        _ => {}
                    },
                    Err(err) => {
                        error!("Error decoding GPSD data: {}", err);
                    }
                }

                Poll::Ready(Some(None))
            }
            _ => Poll::Ready(None),
        }
    }
}

pub fn tpv_to_waypoint(tpv: &Tpv) -> Option<Waypoint> {
    // TODO also support 2D fix
    if let Tpv {
        mode: Mode::Fix3d,
        time: Some(time_str),
        lat: Some(lat),
        lon: Some(lon),
        alt: Some(alt),
        speed: Some(speed),
        ..
    } = tpv
    {
        let mut wp = Waypoint::new(Point::new(*lon, *lat));
        wp.time =
            Some(DateTime::parse_from_rfc3339(time_str).expect("Could not parse timestamp").into());
        wp.elevation = Some(*alt as f64);
        wp.speed = Some(*speed as f64);

        Some(wp)
    } else {
        None
    }
}
