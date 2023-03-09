use log::{error, info};
extern crate google_sheets4 as sheets4;
use sheets4::api::{
    AddSheetRequest, BatchUpdateSpreadsheetRequest, Request, SheetProperties, ValueRange,
};
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

#[mockall::automock]
#[async_trait::async_trait]
pub trait SpreadsheetTrait {
    async fn connect(secrets_file: &str, spreadsheet_id: &str) -> Self;
    async fn sheet_exists(&self, title: &str) -> bool;
    async fn create_sheet(&self, title: &str);
    async fn append<'a>(&'a self, sheet: &'a str, data: Vec<Vec<String>>);
}

pub struct Spreadsheet {
    hub: Hub,
    spreadsheet_id: String,
}

#[async_trait::async_trait]
impl SpreadsheetTrait for Spreadsheet {
    async fn connect(secrets_file: &str, spreadsheet_id: &str) -> Self {
        let secret = oauth2::read_service_account_key(secrets_file)
            .await
            .expect("user secret");

        let auth = oauth2::ServiceAccountAuthenticator::builder(secret)
            .build()
            .await
            .expect("authenticator");

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

    async fn sheet_exists(&self, title: &str) -> bool {
        let result = self
            .hub
            .spreadsheets()
            .get(self.spreadsheet_id.as_str())
            .doit()
            .await
            .unwrap();

        if let Some(sheets) = result.1.sheets {
            sheets.iter().any(|sheet| {
                if let Some(props) = &sheet.properties {
                    matches!(&props.title, Some(t) if t == title)
                } else {
                    false
                }
            })
        } else {
            false
        }
    }

    async fn create_sheet(&self, title: &str) {
        let req = BatchUpdateSpreadsheetRequest {
            requests: Some(vec![Request {
                add_sheet: Some(AddSheetRequest {
                    properties: Some(SheetProperties {
                        title: Some(title.to_string()),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            }]),
            ..Default::default()
        };

        self.hub
            .spreadsheets()
            .batch_update(req, self.spreadsheet_id.as_str())
            .doit()
            .await
            .unwrap();
    }

    async fn append<'a>(&'a self, sheet: &'a str, data: Vec<Vec<String>>) {
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
