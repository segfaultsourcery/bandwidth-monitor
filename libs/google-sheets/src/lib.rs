use async_trait::async_trait;
use log::{error, info};
extern crate google_sheets4 as sheets4;
use sheets4::api::ValueRange;
use sheets4::Error;
use sheets4::{hyper, hyper_rustls, oauth2, Sheets};
use std::default::Default;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum BandwidthMonitorError {
    #[error("Unknown bandwidth_monitor Error")]
    Unknown,
}

pub type ResultT<T> = Result<T, BandwidthMonitorError>;

pub type Hub = Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

#[async_trait]
pub trait Append {
    async fn append(&self, sheet: &str, data: Vec<Vec<String>>);
}

pub struct Spreadsheet {
    hub: Hub,
    spreadsheet_id: String,
}

impl Spreadsheet {
    pub async fn connect(secrets_file: &str, spreadsheet_id: &str) -> Self {
        let secret = oauth2::read_application_secret(secrets_file).await.unwrap();

        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk("auth_cache.json")
        .build()
        .await
        .unwrap();

        let hub = Sheets::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            auth,
        );

        Spreadsheet {
            hub,
            spreadsheet_id: spreadsheet_id.to_string(),
        }
    }
}

#[async_trait]
impl Append for Spreadsheet {
    async fn append(&self, sheet: &str, data: Vec<Vec<String>>) {
        // As the method needs a request, you would usually fill it with the desired information
        // into the respective structure. Some of the parts shown here might not be applicable !
        // Values shown here are possibly random and not representative !
        let req = ValueRange {
            values: Some(data),
            ..Default::default()
        };

        // You can configure optional parameters by calling the respective setters at will, and
        // execute the final call using `doit()`.
        // Values shown here are possibly random and not representative !
        let result = self
            .hub
            .spreadsheets()
            .values_append(
                req,
                self.spreadsheet_id.as_str(),
                format!("'{sheet}'!A:A").as_str(),
            )
            .value_input_option("USER_ENTERED")
            .response_value_render_option("UNFORMATTED_VALUE")
            .insert_data_option("INSERT_ROWS")
            .doit()
            .await;

        match result {
            Err(e) => match e {
                // The Error enum provides details about what exactly happened.
                // You can also just use its `Debug`, `Display` or `Error` traits
                Error::HttpError(_)
                | Error::Io(_)
                | Error::MissingAPIKey
                | Error::MissingToken(_)
                | Error::Cancelled
                | Error::UploadSizeLimitExceeded(_, _)
                | Error::Failure(_)
                | Error::BadRequest(_)
                | Error::FieldClash(_)
                | Error::JsonDecodeError(_, _) => error!("{}", e),
            },
            Ok(res) => info!("Success: {:?}", res),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 10), Ok(11));
//     }
// }
