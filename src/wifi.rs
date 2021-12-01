use lazy_static::lazy_static;

use macaddr::MacAddr6;

use regex::Regex;

use serde::{Deserialize, Serialize};

use serde_with::DisplayFromStr;

use tokio::process::Command;

/// Wi-Fi device as defined by nmcli tables
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiDevice {
    #[serde_as(as = "DisplayFromStr")]
    bssid: MacAddr6,
    ssid: Option<String>,
    mode: WifiDeviceMode,
    chan: u32,
    rate: String, // TODO use f32
    signal: u32,
    security: String, // TODO store as vec enums/strings?
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum WifiDeviceMode {
    #[serde(rename = "Infra")]
    Infrastructure,
    #[serde(rename = "Ad-Hoc")]
    AdHoc,
}

pub async fn scan_devices() -> crate::Result<Vec<WifiDevice>> {
    let out = Command::new("nmcli")
        .args(&[
            "--get-values=BSSID,SSID,MODE,CHAN,RATE,SIGNAL,SECURITY",
            "--colors=no",
            "--escape=no", // TODO change back when the BSSID field issue is fixed
            "device",
            "wifi",
            "list",
            "--rescan",
            "yes",
        ])
        .output()
        .await?;

    let stdout = String::from_utf8(out.stdout).expect("Could not parse nmcli output");

    // HACK Unfortunately the csv crate does not play nicely with escaped
    // delimiters, so for now we will just fix the BSSID field manually by
    // quoting it.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^[0-9A-Z]{2}(:[0-9A-Z]{2}){5}").unwrap();
    }
    let data = RE.replace_all(&stdout, "\"$0\"");

    #[rustfmt::skip]
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b':')
        .has_headers(false)
        .from_reader(data.as_bytes());

    reader.deserialize().into_iter().map(|x| x.map_err(Into::into)).collect()
}
