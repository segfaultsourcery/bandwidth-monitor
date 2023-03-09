use chrono::{DateTime, Local};
use log::error;
use serde::{Deserialize, Serialize};
use std::process::Command;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum BandwidthMonitorError {
    #[error("Unknown bandwidth_monitor Error")]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub id: u32,
    pub host: String,
    pub name: String,
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerList {
    pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    pub jitter: f32,
    pub latency: f32,
    pub low: f32,
    pub high: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Latency {
    pub iqm: f32,
    pub low: f32,
    pub high: f32,
    pub jitter: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bandwidth {
    pub bandwidth: u32,
    pub bytes: u32,
    pub elapsed: u32,
    pub latency: Latency,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResult {
    pub timestamp: DateTime<Local>,
    pub ping: Ping,
    pub download: Bandwidth,
    pub upload: Bandwidth,
    #[serde(rename = "packetLoss")]
    pub packet_loss: Option<f32>,
}

pub type ResultT<T> = Result<T, BandwidthMonitorError>;

pub fn fetch_near_test_servers() -> ServerList {
    let output = Command::new("/usr/bin/speedtest")
        .arg("--servers")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    serde_json::from_str(stdout.as_str()).unwrap()
}

pub fn test_bandwidth(server: &Server) -> TestResult {
    let output = Command::new("/usr/bin/speedtest")
        .arg("--server-id")
        .arg(server.id.to_string())
        .arg("--format")
        .arg("json")
        .arg("-A")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    error!("{stdout}");

    serde_json::from_str(stdout.as_str()).unwrap()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn hello_world_error() {
//         assert_eq!(
//             hello("World"),
//             Err(BandwidthMonitorError::HelloWorldError("World".to_string()))
//         );
//         assert_eq!(
//             hello("wOrld"),
//             Err(BandwidthMonitorError::HelloWorldError("wOrld".to_string()))
//         );
//     }

//     #[test]
//     fn hello_world_success() {
//         assert_eq!(hello("Semptic"), Ok(()));
//     }
// }
