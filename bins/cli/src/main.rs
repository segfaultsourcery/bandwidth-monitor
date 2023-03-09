extern crate ansi_term;
extern crate clap_verbosity_flag;
extern crate loggerv;

use anyhow::{Context, Result};

use bandwidth_monitor_google_sheets::Spreadsheet;
use bandwidth_monitor_ookla_speedtest::{fetch_near_test_servers, test_bandwidth, Server};
use log::{debug, info};

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    setup(&args).context("Failed to setup application environment")?;

    let spreadsheet = Spreadsheet::connect(&args.client_secrets_file, &args.spreadsheet_id).await;
    let near_servers = fetch_near_test_servers();

    for server in near_servers.servers {
        test_and_store(&spreadsheet, &server).await;
    }

    Ok(())
}

async fn test_and_store(spreadsheet: &Spreadsheet, server: &Server) {
    info!("Testing {}", server.name);

    let result = test_bandwidth(server);

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
