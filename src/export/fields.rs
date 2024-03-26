use std::collections::HashMap;

use crate::cli::config::Dataset;
use anyhow::Error;
use anyhow::Result;
use polars::prelude::{NamedFrom, Series};
use serde_json::Value;
use utils::utils::hex_str_to_u64;
#[derive(Debug)]
pub enum FieldData {
    BlocksData(BlockFieldData),
    TransactionsData(TransactionsFieldData),
    LogsData(LogFieldData),
}
#[derive(Debug)]
pub enum LogFieldData {
    Id(Vec<String>),
    LogIndex(Vec<u64>),
    TransactionIndex(Vec<u64>),
    TransactionHash(Vec<String>),
    //BlockHash(Vec<String>),
    Address(Vec<String>),
    Data(Vec<String>),
    Topics(Vec<Vec<String>>),
}
#[derive(Debug)]
pub enum BlockFieldData {
    Hash(Vec<String>),
    Number(Vec<u64>),
    ParentHash(Vec<String>),
    Timestamp(Vec<u64>),
    Miner(Vec<String>),
    StateRoot(Vec<String>),
    TransactionsRoot(Vec<String>),
    ReceiptsRoot(Vec<String>),
    GasUsed(Vec<u64>),
    ExtraData(Vec<String>),
    BaseFeePerGas(Vec<u64>),
    LogsBloom(Vec<String>),
    TotalDifficulty(Vec<u64>),
    Size(Vec<u64>),
}
#[derive(Debug)]
pub enum TransactionsFieldData {
    Id(Vec<String>),
    TransactionIndex(Vec<u64>),
    From(Vec<String>),
    To(Vec<String>),
    Hash(Vec<String>),
    Gas(Vec<u64>),
    GasPrice(Vec<u64>),
    MaxFeePerGas(Vec<u64>),
    MaxPriorityFeePerGas(Vec<u64>),
    Input(Vec<String>),
    Nonce(Vec<u64>),
    Value(Vec<String>),
    V(Vec<u64>),
    R(Vec<String>),
    S(Vec<String>),
    YParity(Vec<u64>),
    ChainId(Vec<u64>),
    GasUsed(Vec<u64>),
    CumulativeGasUsed(Vec<u64>),
    EffectiveGasPrice(Vec<u64>),
    ContractAddress(Vec<String>),
    Type(Vec<u64>),
    Status(Vec<u64>),
    Sighash(Vec<String>),
}

impl FieldData {
    pub fn add_value(&mut self, value: &Value) -> Result<()> {
        match self {
            FieldData::BlocksData(_data) => self.add_blocks_value(value),
            FieldData::TransactionsData(_data) => self.add_transactions_value(value),
            FieldData::LogsData(_data) => self.add_logs_value(value),
            //logs
            //traces
            //_ => panic!("Unsupported type"),
        }
    }

    pub fn add_logs_value(&mut self, value: &Value) -> Result<()> {
        match self {
            Self::LogsData(data) => {
                match data {
                    LogFieldData::Id(vec) | LogFieldData::TransactionHash(vec) => {
                        let string_value = value
                            .as_str()
                            .ok_or_else(|| Error::msg("Expected a string"))?;
                        vec.push(string_value.to_string());
                    }
                    LogFieldData::LogIndex(vec) | LogFieldData::TransactionIndex(vec) => {
                        let number_value = value
                            .as_u64()
                            .ok_or_else(|| Error::msg("Expected a u64 number"))?;
                        vec.push(number_value);
                    }

                    // Add other cases as necessary
                    LogFieldData::Address(vec) | LogFieldData::Data(vec) => {
                        let string_value = value
                            .as_str()
                            .ok_or_else(|| Error::msg("Expected a string"))?;
                        vec.push(string_value.to_string());
                    }

                    LogFieldData::Topics(vec) => {
                        let topics_array = value
                            .as_array()
                            .ok_or_else(|| Error::msg("Expected an array"))?;
                        let mut topics = Vec::new();
                        for topic in topics_array {
                            let topic_str = topic
                                .as_str()
                                .ok_or_else(|| Error::msg("Expected a string in array"))?;
                            topics.push(topic_str.to_string());
                        }
                        vec.push(topics);
                    }
                }
                Ok(())
            }
            _ => Err(Error::msg("Unsupported logs type")),
        }
    }

    pub fn add_blocks_value(&mut self, value: &Value) -> Result<()> {
        match self {
            Self::BlocksData(data) => {
                match data {
                    BlockFieldData::Hash(vec)
                    | BlockFieldData::ParentHash(vec)
                    | BlockFieldData::Miner(vec)
                    | BlockFieldData::StateRoot(vec)
                    | BlockFieldData::TransactionsRoot(vec)
                    | BlockFieldData::ReceiptsRoot(vec)
                    | BlockFieldData::ExtraData(vec)
                    | BlockFieldData::LogsBloom(vec) => {
                        let string_value = value
                            .as_str()
                            .ok_or_else(|| Error::msg("Expected a string"))?;
                        vec.push(string_value.to_string());
                    }
                    BlockFieldData::Number(vec) => {
                        let number_value = value
                            .as_u64()
                            .ok_or_else(|| Error::msg("Expected a u64 number"))?;
                        vec.push(number_value);
                    }
                    //BlockFieldData::Number(vec)
                    //| BlockFieldData::GasUsed(vec)
                    BlockFieldData::TotalDifficulty(vec) | BlockFieldData::Size(vec) => {
                        let number_value = value
                            .as_u64()
                            .ok_or_else(|| Error::msg("Expected a u64 number"))?;
                        vec.push(number_value);
                    }
                    BlockFieldData::GasUsed(vec) => {
                        let str_value = value
                            .as_str()
                            .ok_or_else(|| Error::msg("Expected a u64 number"))?;
                        let number_value = hex_str_to_u64(str_value)?;
                        vec.push(number_value);
                    }

                    BlockFieldData::Timestamp(vec) => {
                        let timestamp_value = value
                            .as_f64()
                            .ok_or_else(|| Error::msg("Expected a f64 number"))?
                            as u64;
                        vec.push(timestamp_value);
                    }
                    BlockFieldData::BaseFeePerGas(vec) => {
                        // Use a default value of 0 if the conversion fails
                        let base_fee = value.as_u64().unwrap_or(0);
                        vec.push(base_fee);
                    }
                }
                Ok(())
            }
            _ => Err(Error::msg("Unsupported type")),
        }
    }

    pub fn add_transactions_value(&mut self, value: &Value) -> Result<()> {
        match self {
            Self::TransactionsData(data) => {
                match data {
                    TransactionsFieldData::Id(vec)
                    | TransactionsFieldData::From(vec)
                    | TransactionsFieldData::Hash(vec)
                    | TransactionsFieldData::Input(vec)
                    | TransactionsFieldData::R(vec)
                    | TransactionsFieldData::S(vec)
                    | TransactionsFieldData::ContractAddress(vec)
                    | TransactionsFieldData::Sighash(vec) => {
                        let string_value = value
                            .as_str()
                            .ok_or_else(|| Error::msg("Expected a string"))?;
                        vec.push(string_value.to_string());
                    }
                    TransactionsFieldData::TransactionIndex(vec)
                    | TransactionsFieldData::Gas(vec)
                    | TransactionsFieldData::GasPrice(vec)
                    | TransactionsFieldData::Nonce(vec)
                    | TransactionsFieldData::YParity(vec)
                    | TransactionsFieldData::ChainId(vec)
                    //| TransactionsFieldData::GasUsed(vec)
                    | TransactionsFieldData::CumulativeGasUsed(vec)
                    | TransactionsFieldData::EffectiveGasPrice(vec)
                    | TransactionsFieldData::Type(vec)
                    | TransactionsFieldData::Status(vec) => {
                        let number_value = value
                            .as_u64()
                            .ok_or_else(|| Error::msg("Expected a u64 number"))?;
                        vec.push(number_value);
                    },
                    TransactionsFieldData::GasUsed(vec) => {
                        let str_value = value
                            .as_str()
                            .ok_or_else(|| Error::msg("Expected a u64 number"))?;
                        let number_value = hex_str_to_u64(str_value)?;
                        vec.push(number_value);
                    },

                    TransactionsFieldData::To(vec) | TransactionsFieldData::Value(vec) => {
                        // Use a default empty string if conversion fails
                        let string_value = value.as_str().unwrap_or("").to_string();
                        vec.push(string_value);
                    }
                    TransactionsFieldData::MaxFeePerGas(vec)
                    | TransactionsFieldData::MaxPriorityFeePerGas(vec)
                    | TransactionsFieldData::V(vec) => {
                        // Use a default value of 0 if the conversion fails
                        let number_value = value.as_u64().unwrap_or(0);
                        vec.push(number_value);
                    }
                }
                Ok(())
            }
            _ => Err(Error::msg("Unsupported type")),
        }
    }
}

pub fn create_field_data(field: &str, dataset: Dataset) -> Result<FieldData> {
    match dataset {
        Dataset::Blocks => create_block_field_data(field),
        Dataset::Transactions => create_transaction_field_data(field),
        Dataset::Logs => create_log_field_data(field),
        //_ => panic!("Dataset not found"),
    }
}

macro_rules! create_block_field_data {
    ($variant:ident) => {
        FieldData::BlocksData(BlockFieldData::$variant(vec![]))
    };
}

macro_rules! create_log_field_data {
    ($variant:ident) => {
        FieldData::LogsData(LogFieldData::$variant(vec![]))
    };
}

macro_rules! create_transaction_field_data {
    ($variant:ident) => {
        FieldData::TransactionsData(TransactionsFieldData::$variant(vec![]))
    };
}

fn create_transaction_field_data(field: &str) -> Result<FieldData> {
    match field {
        "id" => Ok(create_transaction_field_data!(Id)),
        "transactionIndex" => Ok(create_transaction_field_data!(TransactionIndex)),
        "from" => Ok(create_transaction_field_data!(From)),
        "to" => Ok(create_transaction_field_data!(To)),
        "hash" => Ok(create_transaction_field_data!(Hash)),
        "gas" => Ok(create_transaction_field_data!(Gas)),
        "gasPrice" => Ok(create_transaction_field_data!(GasPrice)),
        "maxFeePerGas" => Ok(create_transaction_field_data!(MaxFeePerGas)),
        "maxPriorityFeePerGas" => Ok(create_transaction_field_data!(MaxPriorityFeePerGas)),
        "input" => Ok(create_transaction_field_data!(Input)),
        "nonce" => Ok(create_transaction_field_data!(Nonce)),
        "value" => Ok(create_transaction_field_data!(Value)),
        "v" => Ok(create_transaction_field_data!(V)),
        "r" => Ok(create_transaction_field_data!(R)),
        "s" => Ok(create_transaction_field_data!(S)),
        "yParity" => Ok(create_transaction_field_data!(YParity)),
        "chainId" => Ok(create_transaction_field_data!(ChainId)),
        "gasUsed" => Ok(create_transaction_field_data!(GasUsed)),
        "cumulativeGasUsed" => Ok(create_transaction_field_data!(CumulativeGasUsed)),
        "effectiveGasPrice" => Ok(create_transaction_field_data!(EffectiveGasPrice)),
        "contractAddress" => Ok(create_transaction_field_data!(ContractAddress)),
        "type" => Ok(create_transaction_field_data!(Type)),
        "status" => Ok(create_transaction_field_data!(Status)),
        "sighash" => Ok(create_transaction_field_data!(Sighash)),
        _ => Err(Error::msg(format!("Field '{}' not found", field))),
    }
}

fn create_log_field_data(field: &str) -> Result<FieldData> {
    match field {
        "id" => Ok(create_log_field_data!(Id)),
        "logIndex" => Ok(create_log_field_data!(LogIndex)),
        "transactionIndex" => Ok(create_log_field_data!(TransactionIndex)),
        "transactionHash" => Ok(create_log_field_data!(TransactionHash)),
        // "blockHash" => Ok(create_log_field_data!(BlockHash)),
        "address" => Ok(create_log_field_data!(Address)),
        "data" => Ok(create_log_field_data!(Data)),
        "topics" => Ok(create_log_field_data!(Topics)),
        _ => Err(Error::msg(format!("Field '{}' not found", field))),
    }
}

fn create_block_field_data(field: &str) -> Result<FieldData> {
    match field {
        "hash" => Ok(create_block_field_data!(Hash)),
        "number" => Ok(create_block_field_data!(Number)),
        "parentHash" => Ok(create_block_field_data!(ParentHash)),
        "timestamp" => Ok(create_block_field_data!(Timestamp)),
        "miner" => Ok(create_block_field_data!(Miner)),
        "stateRoot" => Ok(create_block_field_data!(StateRoot)),
        "transactionsRoot" => Ok(create_block_field_data!(TransactionsRoot)),
        "receiptsRoot" => Ok(create_block_field_data!(ReceiptsRoot)),
        "gasUsed" => Ok(create_block_field_data!(GasUsed)),
        "extraData" => Ok(create_block_field_data!(ExtraData)),
        "baseFeePerGas" => Ok(create_block_field_data!(BaseFeePerGas)),
        "logsBloom" => Ok(create_block_field_data!(LogsBloom)),
        "totalDifficulty" => Ok(create_block_field_data!(TotalDifficulty)),
        "size" => Ok(create_block_field_data!(Size)),
        _ => Err(Error::msg(format!("Field '{}' not found", field))),
    }
}

pub fn create_columns_from_field_data(
    field_map: &HashMap<String, FieldData>,
    fields: &[&str],
    //data: FieldData,
) -> Vec<Series> {
    let mut columns: Vec<Series> = vec![];
    //get dataset type here
    fields.iter().for_each(|field| match field_map.get(*field) {
        Some(FieldData::BlocksData(data)) => {
            match data {
                BlockFieldData::Hash(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::Number(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::ParentHash(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::Timestamp(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::Miner(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::StateRoot(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::TransactionsRoot(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::ReceiptsRoot(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::GasUsed(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::ExtraData(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::BaseFeePerGas(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::LogsBloom(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::TotalDifficulty(vec) => columns.push(Series::new(*field, vec)),
                BlockFieldData::Size(vec) => columns.push(Series::new(*field, vec)),
                //_ => panic!("{} not found", field),
            };
        }

        Some(FieldData::TransactionsData(data)) => match data {
            TransactionsFieldData::Id(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::TransactionIndex(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::From(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::To(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::Hash(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::Gas(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::GasPrice(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::MaxFeePerGas(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::MaxPriorityFeePerGas(vec) => {
                columns.push(Series::new(*field, vec))
            }
            TransactionsFieldData::Input(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::Nonce(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::Value(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::V(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::R(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::S(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::YParity(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::ChainId(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::GasUsed(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::CumulativeGasUsed(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::EffectiveGasPrice(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::ContractAddress(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::Type(vec) => columns.push(Series::new(*&field, vec)),
            TransactionsFieldData::Status(vec) => columns.push(Series::new(*field, vec)),
            TransactionsFieldData::Sighash(vec) => columns.push(Series::new(*field, vec)),
        },
        // Some(FieldData::LogsData(data)) => match data {
        //     LogFieldData::Id(vec) => columns.push(Series::new(*field, vec)),
        //     LogFieldData::LogIndex(vec) => columns.push(Series::new(*field, vec)),
        //     LogFieldData::TransactionIndex(vec) => columns.push(Series::new(*field, vec)),
        //     LogFieldData::TransactionHash(vec) => columns.push(Series::new(*field, vec)),
        //     //LogFieldData::BlockHash(vec) => columns.push(Series::new(*field, vec)),
        //     LogFieldData::Address(vec) => columns.push(Series::new(*field, vec)),
        //     LogFieldData::Data(vec) => columns.push(Series::new(*field, vec)),
        //     LogFieldData::Topics(vec) => {
        //         let series_list: Vec<_> = vec
        //             .into_iter()
        //             .map(|v| {
        //                 // Convert Vec<String> to Series
        //                 Series::new("inner_series", v)
        //             })
        //             .collect();
        //         // Convert the list of Series into a ListChunked
        //         let list_series = Series::new("topics", series_list);
        //         columns.push(list_series)
        //     } //check this later
        // },
        Some(FieldData::LogsData(data)) => match data {
            LogFieldData::Id(vec) => {
                if !vec.is_empty() {
                    columns.push(Series::new(*field, vec));
                }
            }
            LogFieldData::LogIndex(vec) => {
                if !vec.is_empty() {
                    columns.push(Series::new(*field, vec));
                }
            }
            LogFieldData::TransactionIndex(vec) => {
                if !vec.is_empty() {
                    columns.push(Series::new(*field, vec));
                }
            }
            LogFieldData::TransactionHash(vec) => {
                if !vec.is_empty() {
                    columns.push(Series::new(*field, vec));
                }
            }
            //LogFieldData::BlockHash(vec) => if !vec.is_empty() { columns.push(Series::new(*field, vec)) },
            LogFieldData::Address(vec) => {
                if !vec.is_empty() {
                    columns.push(Series::new(*field, vec));
                }
            }
            LogFieldData::Data(vec) => {
                if !vec.is_empty() {
                    columns.push(Series::new(*field, vec));
                }
            }
            LogFieldData::Topics(vec) => {
                let series_list: Vec<_> = vec
                    .into_iter()
                    .filter(|v| !v.is_empty()) // Filter out empty vectors
                    .map(|v| {
                        // Convert Vec<String> to Series
                        Series::new("inner_series", v)
                    })
                    .collect();
                if !series_list.is_empty() {
                    // Convert the list of Series into a ListChunked
                    let list_series = Series::new("topics", series_list);
                    columns.push(list_series);
                }
            } //check this later
        },

        _ => panic!("{} not found", field),
    });
    columns
}
