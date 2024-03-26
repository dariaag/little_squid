use crate::cli::config::Dataset;
use crate::export::export::save_to_file;
use anyhow::{Error, Result};
use crossbeam::channel::Receiver;
use serde_json::Value;
use tracing::debug;
pub fn write_loop(
    dataset: Dataset,
    fields: Vec<String>,
    write_rx: Receiver<Vec<Value>>,
) -> Result<()> {
    loop {
        //receive the bytes from stats

        //let buffer = write_rx.recv().unwrap();
        let buffer = match write_rx.recv() {
            Ok(b) => b,
            Err(e) => {
                if e.to_string()
                    .contains("receiving on an empty and disconnected channel")
                {
                    debug!("Channel closed");
                    break;
                } else {
                    return Err(Error::msg(e));
                }
            }
        };
        if buffer.is_empty() {
            debug!("Buffer is empty");
            break;
        }
        save_to_file(dataset, &fields, buffer)?;
    }
    Ok(())
}
