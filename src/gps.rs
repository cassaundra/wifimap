use std::net::SocketAddr;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use futures::prelude::*;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

use gpsd_proto::{Mode, Tpv, UnifiedResponse};

use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

pub struct GpsConnection {
    framed: Framed<TcpStream, LinesCodec>,
}

impl GpsConnection {
    pub async fn connect(addr: &SocketAddr) -> io::Result<Self> {
        let sock = TcpStream::connect(addr).await?;
        let mut framed = Framed::new(sock, LinesCodec::new());

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
        match self.get_mut().framed.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(line))) => {
                match serde_json::from_str(&line) {
                    Ok(rd) => {
                        match rd {
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
                        }
                    }
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
