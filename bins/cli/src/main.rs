extern crate ansi_term;
extern crate clap_verbosity_flag;
extern crate loggerv;

use anyhow::{Context, Result};

use bandwidth_monitor_google_sheets::{Spreadsheet, SpreadsheetTrait};
use bandwidth_monitor_ookla_speedtest::{BandwidthTester, BandwidthTesterTrait, Server};
use log::{debug, info};

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    setup(&args).context("Failed to setup application environment")?;

    let spreadsheet = Spreadsheet::connect(&args.client_secrets_file, &args.spreadsheet_id).await;
    let bandwidth_tester = BandwidthTester();

    let near_servers = bandwidth_tester.fetch_near_test_servers();

    for server in near_servers.servers {
        test_and_store(&bandwidth_tester, &spreadsheet, &server).await;
    }

    Ok(())
}

async fn test_and_store(
    bandwidth_tester: &impl BandwidthTesterTrait,
    spreadsheet: &impl SpreadsheetTrait,
    server: &Server,
) {
    info!("Testing {}", server.name);

    let result = bandwidth_tester.test_bandwidth(server);

    let result_vec = vec![
        result.timestamp.to_rfc3339(),
        server.name.to_string(),
        server.location.to_string(),
        result.packet_loss.unwrap_or(0.0).to_string(),
        result.ping.latency.to_string(),
        (result.download.bandwidth as f32 / 125000.0).to_string(),
        result.download.latency.iqm.to_string(),
        (result.upload.bandwidth as f32 / 125000.0).to_string(),
        result.upload.latency.iqm.to_string(),
    ];

    if !spreadsheet.sheet_exists(server.name.as_str()).await {
        spreadsheet.create_sheet(server.name.as_str()).await;

        let header = vec![
            "Time",
            "Server Name",
            "Server Location",
            "Packet Loss",
            "Idle Latency (ms)",
            "Download (Mbps)",
            "Download Latency (ms)",
            "Upload (Mbps)",
            "Upload Latency (ms)",
        ]
        .iter()
        .map(|head| head.to_string())
        .collect();

        spreadsheet.append(server.name.as_str(), vec![header]).await;
    }
    spreadsheet
        .append(server.name.as_str(), vec![result_vec])
        .await;
}

fn setup(opt: &Args) -> Result<()> {
    // #[cfg(windows)]
    // ansi_term::enable_ansi_support().context("Failed to enable ansi support")?;

    loggerv::Logger::new()
        .max_level(
            opt.verbosity
                .log_level()
                .context("Failed to get log level")?,
        )
        .level(opt.debug)
        .module_path(opt.debug)
        .line_numbers(opt.debug)
        .init()
        .context("Failed to setup logger")?;

    debug!("{:#?}", *opt);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, name = "bandwidth-monitor", about = "This uses Ookla to run speed tests and store the results in a google sheet")]
struct Args {
    #[structopt(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// Enables debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Path to the secrets file provided by google
    #[structopt(short, long, default_value = "secret.json")]
    client_secrets_file: String,

    /// Id of the spreadsheet
    #[structopt()]
    spreadsheet_id: String,
}

#[cfg(test)]
mod tests {
    use bandwidth_monitor_google_sheets::MockSpreadsheetTrait as Spreadsheet;
    use bandwidth_monitor_ookla_speedtest::{
        MockBandwidthTesterTrait as BandwidthTester, TestResult,
    };
    use mockall::Sequence;

    use super::*;

    #[tokio::test]
    async fn ensure_test_and_store_works_properly_for_existing_spreadsheets() {
        let server = Server {
            id: 1,
            host: "host".into(),
            location: "location".into(),
            name: "my_little_server".into(),
        };

        // Mock the spreadsheet.
        let spreadsheet = {
            let server_name = server.name.clone();

            let mut spreadsheet = Spreadsheet::new();

            spreadsheet.expect_sheet_exists().returning(|_| true);

            spreadsheet
                .expect_append()
                .returning(move |supplied_server, supplied_rows| {
                    // check that server and rows are what you expect.

                    assert_eq!(
                        &server_name, supplied_server,
                        "the server name wasn't right"
                    );

                    let expected_rows = vec![vec![
                        "1970-01-01T01:00:00+01:00",
                        &server_name,
                        "location",
                        "0",
                        "0",
                        "0",
                        "0",
                        "0",
                        "0",
                    ]];

                    assert_eq!(
                        expected_rows, supplied_rows,
                        "the supplied rows weren't right"
                    );
                });

            spreadsheet
        };

        // Mock the bandwidth tester.
        let bandwidth_tester = {
            let server_name = server.name.clone();

            let mut bandwidth_tester = BandwidthTester::new();

            bandwidth_tester
                .expect_test_bandwidth()
                .returning(move |server| {
                    assert_eq!(server_name, server.name, "the server name wasn't right");

                    TestResult {
                        timestamp: Default::default(),
                        ping: Default::default(),
                        download: Default::default(),
                        upload: Default::default(),
                        packet_loss: None,
                    }
                });

            bandwidth_tester
        };

        // Run the test.
        test_and_store(&bandwidth_tester, &spreadsheet, &server).await;
    }

    #[tokio::test]
    async fn ensure_test_and_store_works_properly_for_new_spreadsheets() {
        let server = Server {
            id: 1,
            host: "host".into(),
            location: "location".into(),
            name: "my_little_server".into(),
        };

        // Mock the spreadsheet.
        let spreadsheet = {
            let server_name = server.name.clone();

            let mut spreadsheet = Spreadsheet::new();

            spreadsheet.expect_sheet_exists().returning(|_| false);
            spreadsheet.expect_create_sheet().returning(|_| {});

            // The sequence lets us have multiple outcomes for the same function.
            let mut seq = Sequence::new();

            spreadsheet
                .expect_append()
                .times(1)
                .returning(move |_, supplied_rows| {
                    let expected_rows = vec![vec![
                        "Time",
                        "Server Name",
                        "Server Location",
                        "Packet Loss",
                        "Idle Latency (ms)",
                        "Download (Mbps)",
                        "Download Latency (ms)",
                        "Upload (Mbps)",
                        "Upload Latency (ms)",
                    ]];

                    assert_eq!(
                        expected_rows, supplied_rows,
                        "the supplied rows weren't right"
                    );
                })
                .in_sequence(&mut seq);

            spreadsheet
                .expect_append()
                .times(1)
                .returning(move |_, supplied_rows| {
                    let expected_rows = vec![vec![
                        "1970-01-01T01:00:00+01:00",
                        &server_name,
                        "location",
                        "0",
                        "0",
                        "0",
                        "0",
                        "0",
                        "0",
                    ]];

                    assert_eq!(
                        expected_rows, supplied_rows,
                        "the supplied rows weren't right"
                    );
                })
                .in_sequence(&mut seq);

            spreadsheet
        };

        // Mock the bandwidth tester.
        let bandwidth_tester = {
            let server_name = server.name.clone();

            let mut bandwidth_tester = BandwidthTester::new();

            bandwidth_tester
                .expect_test_bandwidth()
                .returning(move |server| {
                    assert_eq!(server_name, server.name, "the server name wasn't right");

                    TestResult {
                        timestamp: Default::default(),
                        ping: Default::default(),
                        download: Default::default(),
                        upload: Default::default(),
                        packet_loss: None,
                    }
                });

            bandwidth_tester
        };

        // Run the test.
        test_and_store(&bandwidth_tester, &spreadsheet, &server).await;
    }
}
