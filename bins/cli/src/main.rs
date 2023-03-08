extern crate ansi_term;
extern crate clap_verbosity_flag;
extern crate loggerv;
use anyhow::{Context, Result};

use bandwidth_monitor_google_sheets::add;
use bandwidth_monitor_ookla_speedtest::hello;
use log::{debug, info};

use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();

    setup(&args).context("Failed to setup application environment")?;

    hello(args.name.as_str()).context("Failed to say hello")?;

    info!("1 + 1 = {}", add(1, 1).context("Failed to add 1 and 1")?);

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

    /// Name to greet.
    #[structopt(name = "NAME")]
    name: String,

    /// Enables debug mode
    #[structopt(short, long)]
    debug: bool,
}
