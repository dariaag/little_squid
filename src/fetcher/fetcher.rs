use anyhow::Error;
use cli::config::Config;
use crossbeam::channel::Sender;
use futures::future::join_all;
use reqwest::{self, Client};
use tracing::debug;

use crate::cli::{
    self,
    config::{Dataset, Range},
};
use anyhow::Result;
use serde_json::{json, Map, Value};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{self, sync::Semaphore};
use utils::{archive::get_worker, utils::sizeof_val};

const MAX_CHUNK_SIZE: usize = 10 * 1024 * 1024;

fn create_query_json(
    dataset: Dataset,
    start_block: u64,
    end_block: u64,
    fields: &[String],
    options: &HashMap<String, Vec<String>>,
) -> Value {
    let field_map = fields
        .iter()
        .map(|field| (field.clone(), json!(true)))
        .collect::<Map<_, _>>();

    match dataset {
        Dataset::Blocks => json!({
            "fields": {"block": field_map},
            "fromBlock": start_block,
            "includeAllBlocks": true,
        }),

        Dataset::Transactions => {
            let options_json = json!(options);
            json!({
                "transactions": [options_json],
                "fields": {
                    "block": {},
                    "transaction": field_map
                },
                "fromBlock": start_block,
                "toBlock": end_block,
                "includeAllBlocks": true,
            })
        }

        Dataset::Logs => {
            let options_json = json!(options);
            json!({
                "logs": [options_json],
                "fields": {
                    "block": {},
                    "transaction": {},
                    "log": field_map
                },
                "fromBlock": start_block,
                "includeAllBlocks": true,
            })
        }
    }
}

pub async fn fetch_block_chunk(
    dataset: Dataset,
    start_block: u64,
    end_block: u64,
    fields: &[String],
    options: &HashMap<String, Vec<String>>,
    client: &Client,
) -> Result<(Vec<Value>, u64), Error> {
    let block_query = create_query_json(dataset, start_block, end_block, fields, options);

    let worker = get_worker(
        "https://v2.archive.subsquid.io/network/ethereum-mainnet",
        &start_block.to_string(),
    )
    .await?;

    let result = client
        .post(worker)
        .json(&block_query)
        .send()
        .await?
        .text()
        .await?;

    let blocks_value: Value = serde_json::from_str(&result)
        .map_err(|e| Error::msg(format!("Error parsing JSON: {}", e)))?;

    let blocks = match blocks_value.as_array() {
        Some(blocks) => blocks,
        None => {
            //println!("Error fetching");
            return Err(Error::msg("Invalid JSON format: Expected an array"));
        }
    };

    let next_block = blocks
        .last()
        .and_then(|b| b["header"]["number"].as_u64())
        .ok_or_else(|| {
            Error::msg("Invalid block data format: 'number' field missing or not a u64")
        })?;
    //println!("Fetched {:?} blocks from {:?}", blocks.len(), start_block);
    if blocks.len() == 1 {}
    Ok((blocks.to_vec(), next_block))
}

fn send_blocks(
    blocks: &[Value],
    write_tx: Sender<Vec<Value>>,
    //stats_tx: &Sender<u64>,
) -> Result<(), Error> {
    let mut data_chunk = Vec::new();
    let mut current_size = 0;
    for data in blocks {
        let chunk_size = sizeof_val(data);
        if current_size + chunk_size >= MAX_CHUNK_SIZE {
            //println!("Sending chunk of size {:?}", data_chunk.len());
            write_tx
                .send(data_chunk.clone())
                .map_err(|e| Error::msg(format!("Error sending blocks: {}", e)))?;
            data_chunk.drain(..);
            current_size = 0;
        }
        //println!("ADDING BLOCKS {:?}", blocks.len());
        current_size += chunk_size;
        // stats_tx
        //     .send(blocks.len() as u64)
        //     .map_err(|e| Error::msg(format!("Error sending stats: {}", e)))?;
        data_chunk.push(data.clone());
    }

    // Send any remaining data
    if !data_chunk.is_empty() {
        //println!("222Sending chunk of size {:?}", data_chunk.len());
        write_tx
            .send(data_chunk)
            .map_err(|e| Error::msg(format!("Error sending blocks: {}", e)))?;
    }

    Ok(())
}

fn compute_chunk_ranges(total_range: &Range, chunk_size: u64) -> Vec<Range> {
    // Divide the total range into smaller ranges of chunk_size
    (total_range.start..total_range.end)
        .step_by(chunk_size as usize)
        .map(|start| Range {
            start: start,
            end: (start + chunk_size).min(total_range.end),
        })
        .collect()
}

pub async fn fetch(
    config: Config,

    write_tx: Sender<Vec<Value>>,
    stats_tx: Sender<u64>,
) -> Result<(), Error> {
    let client = Arc::new(reqwest::Client::new());
    let semaphore = Arc::new(Semaphore::new(10)); // Adjust concurrency level
    fetch_block_ranges(&config, client, semaphore, &write_tx, &stats_tx).await?;
    Ok(())
}

pub async fn fetch_block_ranges(
    config: &Config,
    client: Arc<Client>,
    semaphore: Arc<Semaphore>,
    write_tx: &Sender<Vec<Value>>,
    stats_tx: &Sender<u64>,
) -> Result<(), Error> {
    let ranges = compute_chunk_ranges(&config.range, config.dataset.get_chunk_size());

    let tasks: Vec<_> = ranges
        .into_iter()
        .map(|range| {
            let semaphore_clone = semaphore.clone();
            let client_clone = client.clone();
            let config_clone = config.clone();
            let write_tx_clone = write_tx.clone();
            let stats_tx_clone = stats_tx.clone();

            tokio::spawn(async move {
                let _permit = semaphore_clone.acquire_owned().await.unwrap();
                match fetch_sized_chunk(
                    &config_clone,
                    client_clone,
                    range.start,
                    range.end,
                    &write_tx_clone,
                    //&stats_tx_clone,
                )
                .await
                {
                    Ok(_) => {
                        stats_tx_clone.send(range.end - range.start).unwrap();
                        if range.end == config_clone.range.end {
                            debug!("Finished fetching all blocks");
                            stats_tx_clone.send(0).unwrap(); // Signal the end of the stream
                        }
                    }
                    Err(e) => eprintln!("Error fetching block range: {:?}", e),
                }

                Result::<(), Error>::Ok(())
            })
        })
        .collect();

    let _results = join_all(tasks).await;
    Ok(())
}

pub async fn fetch_sized_chunk(
    config: &Config,
    client: Arc<Client>,
    start_block: u64,
    end_block: u64,
    write_tx: &Sender<Vec<Value>>,
    //stats_tx: &Sender<u64>,
) -> Result<(), Error> {
    let mut current_start = start_block;
    let max_attempts = 3;
    let mut attempt = 0;
    let mut backoff = Duration::from_millis(100);
    let mut fetched_blocks = Vec::new();
    while current_start < end_block {
        if attempt >= max_attempts {
            eprintln!(
                "Max retry attempts reached for blocks starting at {}",
                current_start
            );

            break; // Skip to the next range after max retries
        }

        match fetch_block_chunk(
            config.dataset.clone(),
            current_start,
            end_block,
            &config.fields,
            &config.options,
            &client,
        )
        .await
        {
            Ok((blocks, next_block)) => {
                //println!("Fetched {:?} blocks from {:?}", blocks.len(), current_start);
                fetched_blocks.extend(blocks);
                // let _ = send_blocks(&fetched_blocks, write_tx.clone(), stats_tx);
                current_start = next_block;
                attempt = 0; // Reset attempts after a successful fetch
                backoff = Duration::from_millis(100); // Reset backoff
            }
            Err(e) => {
                eprintln!(
                    "Error fetching blocks: {:?}, retrying in {:?}...",
                    e, backoff
                );
                tokio::time::sleep(backoff).await;
                attempt += 1;
                backoff *= 2; // Exponential backoff
            }
        }
    }
    // println!("Fetched {} blocks, sending", fetched_blocks.len());

    let _ = send_blocks(&fetched_blocks, write_tx.clone());

    Ok(())
}
