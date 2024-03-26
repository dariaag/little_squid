use std::collections::{HashMap, HashSet};

use crate::cli::opts::Opts;
use crate::cli::summaries::print_intro;
use anyhow::{anyhow, Ok, Result};
//use utils::archive::get_height;

#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dataset {
    Blocks,
    Transactions,
    Logs,
}
#[derive(Debug, Clone)]
pub struct Config {
    pub dataset: Dataset,
    pub range: Range,
    pub fields: Vec<String>,
    pub options: HashMap<String, Vec<String>>,
}

impl Dataset {
    pub fn get_name(&self) -> &str {
        match self {
            Dataset::Blocks => "blocks",
            Dataset::Transactions => "transactions",
            Dataset::Logs => "logs",
        }
    }
    pub fn get_chunk_size(&self) -> u64 {
        match self {
            Dataset::Blocks => 10000,
            Dataset::Transactions => 1000,
            Dataset::Logs => 5000,
        }
    }
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;
    fn try_from(opts: Opts) -> Result<Self> {
        let dataset = get_dataset(opts.dataset)?;
        let range = get_range(opts.range)?.try_into()?;
        let fields = get_fields(opts.fields, dataset)?;
        let options = get_options(opts.options, dataset.clone())?;
        print_intro(
            dataset,
            &fields,
            &range,
            &options.values().flatten().collect::<Vec<&String>>(),
        );
        Ok(Config {
            dataset,
            range,
            fields,
            options,
        })
    }
}

impl TryFrom<String> for Dataset {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "blocks" => Ok(Dataset::Blocks),
            "transactions" => Ok(Dataset::Transactions),
            "logs" => Ok(Dataset::Logs),
            _ => Err(anyhow!("Invalid dataset")),
        }
    }
}

impl TryFrom<Vec<String>> for Range {
    type Error = anyhow::Error;
    fn try_from(value: Vec<String>) -> Result<Self> {
        if value.len() == 0 {
            return Ok(Range {
                start: 1,
                end: 18000000,
            });
        }
        let start = value[0].parse::<u64>()?;
        let end = value[1].parse::<u64>()?;
        Ok(Range { start, end })
    }
}

//TODO make default range equal for archive height
fn get_range(range: Option<String>) -> Result<Vec<String>> {
    match range {
        Some(range) => {
            if range.len() == 0 {
                let default_range = vec!["1".to_owned(), "18000000".to_owned()];
                Ok(default_range)
            } else {
                Ok(range.split(":").map(String::from).collect())
            }
        }
        None => {
            // let archive_height =
            //     get_height("https://v2.archive.subsquid.io/network/ethereum-mainnet/height")
            //         .await?;
            let default_range = vec!["1".to_owned(), "18000000".to_owned()];
            Ok(default_range)
        }
    }
}

fn get_fields(fields: Option<Vec<String>>, dataset: Dataset) -> Result<Vec<String>> {
    match dataset {
        Dataset::Blocks => {
            let mut default_fields = HashSet::from([
                "hash".to_owned(),
                "number".to_owned(),
                "timestamp".to_owned(),
                "miner".to_owned(),
                "gasUsed".to_owned(),
                "extraData".to_owned(),
                "baseFeePerGas".to_owned(),
            ]);

            if let Some(additional_fields) = fields {
                additional_fields.into_iter().for_each(|field| {
                    default_fields.insert(field);
                });
            }
            //println!("{:?}", default_fields.into_iter().collect::<Vec<String>>());
            verify_block_fields(default_fields.into_iter().collect::<Vec<String>>())
        }
        Dataset::Transactions => {
            let mut default_fields = HashSet::from([
                "hash".to_owned(),
                "from".to_owned(),
                "to".to_owned(),
                "input".to_owned(),
                "value".to_owned(),
            ]);
            if let Some(additional_fields) = fields {
                additional_fields.into_iter().for_each(|field| {
                    default_fields.insert(field);
                });
            }

            verify_transaction_fields(default_fields.into_iter().collect::<Vec<String>>())
        }
        Dataset::Logs => {
            let mut default_fields = HashSet::from([
                "transactionHash".to_owned(),
                "logIndex".to_owned(),
                "transactionIndex".to_owned(),
                "address".to_owned(),
                "data".to_owned(),
                "topics".to_owned(),
            ]);
            if let Some(additional_fields) = fields {
                additional_fields.into_iter().for_each(|field| {
                    default_fields.insert(field);
                });
            }
            verify_log_fields(default_fields.into_iter().collect::<Vec<String>>())
        }
    }
}
fn verify_transaction_fields(fields: Vec<String>) -> Result<Vec<String>> {
    let valid_fields: &[&str] = &[
        "id",
        "transactionIndex",
        "from",
        "to",
        "hash",
        "gas",
        "gasPrice",
        "maxFeePerGas",
        "maxPriorityFeePerGas",
        "input",
        "nonce",
        "value",
        "v",
        "r",
        "s",
        "yParity",
        "chainId",
        "gasUsed",
        "cumulativeGasUsed",
        "effectiveGasPrice",
        "contractAddress",
        "type",
        "status",
        "sighash",
        "blockHash",
        "blockNumber",
        "timestamp",
        "type",
    ];

    fields
        .into_iter()
        .map(|field| {
            if valid_fields.contains(&field.as_str()) {
                Ok(field)
            } else {
                Err(anyhow!("Invalid field: {}", field))
            }
        })
        .collect()
}

fn verify_block_fields(fields: Vec<String>) -> Result<Vec<String>> {
    let valid_fields: &[&str] = &[
        "hash",
        "number",
        "parentHash",
        "timestamp",
        "miner",
        "stateRoot",
        "transactionsRoot",
        "receiptsRoot",
        "gasUsed",
        "extraData",
        "baseFeePerGas",
        "logsBloom",
        "totalDifficulty",
        "size",
    ];

    fields
        .into_iter()
        .map(|field| {
            if valid_fields.contains(&field.as_str()) {
                Ok(field)
            } else {
                Err(anyhow!("Invalid field: {}", field))
            }
        })
        .collect()
}

fn verify_log_fields(fields: Vec<String>) -> Result<Vec<String>> {
    let valid_fields: &[&str] = &[
        "transactionHash",
        "logIndex",
        "transactionIndex",
        "address",
        "data",
        "topics",
    ];

    fields
        .into_iter()
        .map(|field| {
            if valid_fields.contains(&field.as_str()) {
                Ok(field)
            } else {
                Err(anyhow!("Invalid field: {}", field))
            }
        })
        .collect()
}

fn get_dataset(dataset: Option<String>) -> Result<Dataset> {
    match dataset {
        Some(dataset) => match dataset.as_str() {
            "blocks" => Ok(Dataset::Blocks),
            "transactions" => Ok(Dataset::Transactions),
            "logs" => Ok(Dataset::Logs),
            _ => Err(anyhow!("Invalid dataset")),
        },
        None => Err(anyhow!("No dataset specified")),
    }
}

fn get_options(
    options: Option<Vec<String>>,
    dataset: Dataset,
) -> Result<HashMap<String, Vec<String>>, anyhow::Error> {
    match options {
        Some(options) => {
            if dataset == Dataset::Blocks || options.is_empty() {
                return Ok(HashMap::new());
            }
            let verified_options = get_verified_options(dataset).unwrap();
            let mut options_map: HashMap<String, Vec<String>> = HashMap::new();
            for option in options {
                let option_value = option.split(":").collect::<Vec<&str>>();
                if verified_options.contains(&option_value[0].to_string()) {
                    options_map.insert(
                        option_value[0].to_string(),
                        vec![option_value[1].to_string()], //add mult append
                    );
                } else {
                    return Err(anyhow!("Invalid option"));
                }
            }
            Ok(options_map)
        }
        None => Ok(HashMap::new()),
    }
}

fn get_verified_options(dataset: Dataset) -> Option<Vec<String>> {
    match dataset {
        Dataset::Blocks => Some(vec!["".to_owned()]),
        Dataset::Transactions => Some(vec![
            "from".to_string(),
            "to".to_string(),
            "sighash".to_string(),
        ]),
        Dataset::Logs => Some(vec![
            "address".to_string(),
            "topic0".to_string(),
            "topic1".to_string(),
            "topic2".to_string(),
            "topic3".to_string(),
        ]),
        //_ => None,
    }
}

#[cfg(test)]
mod tests {

    use super::{Config, Dataset, Range};
    use crate::cli::opts::Opts;
    use anyhow::Result;

    #[test]
    fn test_blocks() -> Result<()> {
        let opts: Config = Opts {
            dataset: Some("blocks".to_owned()),
            range: Some("1:10".to_owned()),
            fields: Some(vec!["timestamp".to_owned()]),
            options: Some(vec!["".to_owned()]),
        }
        .try_into()?;
        assert_eq!(opts.dataset, Dataset::Blocks);
        assert_eq!(opts.range, Range { start: 1, end: 10 });
        assert_eq!(opts.fields, vec!["timestamp".to_owned()]);

        return Ok(());
    }
    //TODO add fields vs datasets
    #[test]
    fn test_block_fields() -> Result<()> {
        let opts: Config = Opts {
            dataset: Some("blocks".to_owned()),
            range: Some("1:10000".to_owned()),
            fields: Some(vec![
                "timestamp".to_owned(),
                "miner".to_owned(),
                "logsBloom".to_owned(),
            ]),
            options: Some(vec!["".to_owned()]),
        }
        .try_into()?;
        assert_eq!(opts.dataset, Dataset::Blocks);
        assert_eq!(
            opts.range,
            Range {
                start: 1,
                end: 10000
            }
        );
        assert_eq!(
            opts.fields,
            vec![
                "timestamp".to_owned(),
                "miner".to_owned(),
                "logsBloom".to_owned()
            ]
        );

        return Ok(());
    }
    #[test]
    fn test_transaction_fields() -> Result<()> {
        let opts: Config = Opts {
            dataset: Some("transactions".to_owned()),
            range: Some("1:10000".to_owned()),
            fields: Some(vec!["id".to_owned(), "from".to_owned(), "to".to_owned()]),
            options: Some(vec!["".to_owned()]),
        }
        .try_into()?;
        print!("{:?}", opts);
        assert_eq!(opts.dataset, Dataset::Transactions);
        assert_eq!(
            opts.range,
            Range {
                start: 1,
                end: 10000
            }
        );
        assert_eq!(
            opts.fields,
            vec!["id".to_owned(), "from".to_owned(), "to".to_owned()]
        );
        return Ok(());
    }
}

// #[test]
// fn test_print_key() -> Result<()> {
//     let opts: Config = Opts {
//         args: vec!["foo".to_string()],
//         pwd: None,
//         config: None,
//     }
//     .try_into()?;
//     assert_eq!(opts.operation, Operation::Print(Some("foo".to_string())));

//     return Ok(());
// }
// #[test]
// fn test_add_key_value() -> Result<()> {
//     let opts: Config = Opts {
//         args: vec![
//             String::from("add"),
//             String::from("foo"),
//             String::from("bar"),
//         ],
//         pwd: None,
//         config: None,
//     }
//     .try_into()?;
//     assert_eq!(
//         opts.operation,
//         Operation::Add(String::from("foo"), String::from("bar"))
//     );

//     return Ok(());
// }
// #[test]
// fn test_remove_value() -> Result<()> {
//     let opts: Config = Opts {
//         args: vec![String::from("rm"), String::from("foo")],
//         pwd: None,
//         config: None,
//     }
//     .try_into()?;
//     assert_eq!(opts.operation, Operation::Remove(String::from("foo")));

//     return Ok(());
// }
