use std::io;

use macaddr::MacAddr6;

use serde::{Serialize, Deserialize};

use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiDevice {
    bssid: MacAddr6,
    ssid: String,
    mode: WifiDeviceMode,
    chan: u32,
    rate: f32,
    signal: u32,
    security: Vec<String>, // TODO store as enums?
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum WifiDeviceMode {
    #[serde(rename(serialize = "Infra"))]
    Infrastructure,
    #[serde(rename(serialize = "Ad-Hoc"))]
    AdHoc,
}

pub async fn scan_devices() -> io::Result<Vec<WifiDevice>> {
    let out = Command::new("nmcli")
        .args(&[
            "--get-values=BSSID,SSID,MODE,CHAN,RATE,SIGNAL,SECURITY",
            "--colors=no",
            "device",
            "wifi",
            "list",
            "--rescan",
            "yes",
        ])
        .output().await?;
    let stdout = String::from_utf8(out.stdout).expect("Could not parse nmcli output");
    println!("out: {}", stdout);

    Ok(vec![])
}
