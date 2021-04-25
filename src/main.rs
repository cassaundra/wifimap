#[macro_use]
extern crate log;

use futures::prelude::*;
use futures::sink::SinkExt;

use gpsd_proto::UnifiedResponse;

use tokio::net::TcpStream;

use tokio_util::codec::{Framed, LinesCodec};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr: SocketAddr = "127.0.0.1:2947".parse().unwrap();

    let sock = TcpStream::connect(&addr).await.unwrap();

    let mut framed = Framed::new(sock, LinesCodec::new());
    framed.send(gpsd_proto::ENABLE_WATCH_CMD.to_string()).await.unwrap();
    framed
        .for_each(|line| {
            if let Ok(line) = line {
                match serde_json::from_str(&line) {
                    Ok(rd) => match rd {
                        UnifiedResponse::Version(v) => {
                            if v.proto_major < gpsd_proto::PROTO_MAJOR_MIN {
                                panic!("GPSD major version mismatch");
                            }

                            info!("GPSD version {} connected", v.rev);
                        }
                        UnifiedResponse::Tpv(tpv) => {
                            if let (Some(lat), Some(lon), Some(alt)) = (tpv.lat, tpv.lon, tpv.alt) {
                                println!("lat: {} lon: {} alt: {}", lat, lon, alt);
                            }
                        },
                        _ => {}
                    },
                    Err(err) => {
                        error!("Error decoding GPSD data: {}", err);
                    }
                }
            }
            future::ready(())
        })
        .await;
}
