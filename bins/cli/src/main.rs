extern crate ansi_term;
extern crate clap_verbosity_flag;
extern crate loggerv;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use bandwidth_monitor_google_sheets::{Append, Spreadsheet};
use log::debug;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    setup(&args).context("Failed to setup application environment")?;

    let spreadsheet = Spreadsheet::connect(
        &args.client_secrets_file,
        &args.spreadsheet_id,
    )
    .await;

    spreadsheet
        .append(&args.sheet, vec![vec!["A".to_string(), "B".to_string()]])
        .await;

    Ok(())
}

fn setup(opt: &Args) -> Result<()> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().context("Failed to enable ansi support")?;

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
    #[structopt(short, long, default_value="client_secret.json")]
    client_secrets_file: String,

    /// Id of the spreadsheet
    #[structopt()]
    spreadsheet_id: String,

    /// Name of the sheet 
    #[structopt()]
    sheet: String
}