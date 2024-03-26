//use anyhow::Result as AnyhowResult;
use anyhow::Result;
use clap::Parser;
use crossbeam::channel::unbounded;
use little_squid_cli::cli::config::Config;
use little_squid_cli::cli::opts::Opts;
use little_squid_cli::fetcher::fetcher;
use little_squid_cli::progress::stats;
use little_squid_cli::save;
use std::thread;
use tokio;
#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into().unwrap();
    let fields = config.fields.clone();
    //let start_time = std::time::Instant::now();

    let (write_tx, write_rx) = unbounded();

    let (stat_tx, stat_rx) = unbounded();
    let read_handle = tokio::spawn(fetcher::fetch(config.clone(), write_tx, stat_tx));
    let stats_handle =
        thread::spawn(move || stats::stats_loop(stat_rx, config.range.end - config.range.start));

    let write_handle = thread::spawn(move || save::write_loop(config.dataset, fields, write_rx));

    let read_io_result = read_handle.await?;
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();
    //return error if any thread returned error
    read_io_result?;
    stats_io_result?;
    write_io_result?;
    //let elapsed_time = start_time.elapsed();

    //println!("\n Elapsed time: {:?}", elapsed_time);
    Ok(())
}
