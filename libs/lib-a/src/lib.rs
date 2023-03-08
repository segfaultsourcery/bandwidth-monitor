use log::{debug, info};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum BandwidthMonitorError {
    #[error("I will not say 'Hello, {0}.'!")]
    HelloWorldError(String),
    #[error("Unknown bandwidth_monitor Error")]
    Unknown,
}

pub type ResultT<T> = Result<T, BandwidthMonitorError>;

pub fn hello(name: &str) -> ResultT<()> {
    debug!("Hi, I'm chatty. I want to tell you a great story!");
    if name.to_lowercase() == "world" {
        Err(BandwidthMonitorError::HelloWorldError(name.to_string()))
    } else {
        info!("I'm going to greet now.");
        println!("Hello, {}.", name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_error() {
        assert_eq!(
            hello("World"),
            Err(BandwidthMonitorError::HelloWorldError("World".to_string()))
        );
        assert_eq!(
            hello("wOrld"),
            Err(BandwidthMonitorError::HelloWorldError("wOrld".to_string()))
        );
    }

    #[test]
    fn hello_world_success() {
        assert_eq!(hello("Semptic"), Ok(()));
    }
}
