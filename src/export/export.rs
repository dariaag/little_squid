//use polars::prelude::*;
use crate::cli::config::Dataset;

use polars::prelude::{DataFrame, ParquetCompression, ParquetWriter, Series};
use serde_json::Value;
use std::collections::HashMap;

use crate::export::fields::{create_columns_from_field_data, create_field_data, FieldData};
use anyhow::Error;
use std::fs::{self, File};
use std::path::Path;

fn convert_to_dataframe(
    dataset: Dataset,
    json_data: Vec<Value>,
    fields: Vec<&str>,
) -> Result<DataFrame, Error> {
    let data_fields: Vec<(&str, FieldData)> = fields
        .iter()
        .filter_map(|&field| {
            match create_field_data(field, dataset) {
                Ok(field_data) => Some((field, field_data)),
                Err(_) => None, //todo change to anyhow
            }
        })
        .collect();

    let mut field_map: HashMap<String, FieldData> = data_fields
        .into_iter()
        .map(|(name, data)| (name.to_string(), data))
        .collect();
    //put loop inside func, return mutable reference to fieldmap
    field_map = process_json_object(json_data, field_map, &fields, &dataset).unwrap(); //todo change to anyhow
                                                                                       //create series from fields
    let columns: Vec<Series> = create_columns_from_field_data(&field_map, &fields);

    let df = DataFrame::new(columns)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    Ok(df)
}
//pass fields here
pub fn save_to_file(
    dataset: Dataset,
    fields_vec: &Vec<String>,
    json_data: Vec<Value>,
) -> Result<(), Error> {
    let fields = fields_vec.iter().map(|s| s.as_str()).collect();

    let first_block = json_data
        .first()
        .and_then(|b| b["header"]["number"].as_u64())
        .ok_or_else(|| {
            Error::msg("Invalid block data format: 'number' field missing or not a u64")
        })?;
    let last_block = json_data
        .last()
        .and_then(|b| b["header"]["number"].as_u64())
        .ok_or_else(|| {
            Error::msg("Invalid block data format: 'number' field missing or not a u64")
        })?;

    let mut df = convert_to_dataframe(dataset, json_data, fields)?;
    let folder = Path::new("data");

    if !folder.exists() {
        fs::create_dir_all(folder)?;
    }
    //TODO name file with blocks num and data name
    let file_path = format!(
        "data/{}_{}-{}.parquet",
        dataset.get_name(),
        first_block,
        last_block
    );

    let file =
        File::create(file_path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    ParquetWriter::new(file)
        .with_compression(ParquetCompression::Snappy)
        .finish(&mut df)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}

fn process_json_object(
    json_data: Vec<Value>,
    mut field_map: HashMap<String, FieldData>,
    fields: &[&str],
    dataset: &Dataset,
) -> Result<HashMap<String, FieldData>, Error> {
    for json_obj in json_data {
        match dataset {
            Dataset::Blocks => {
                if let Some(header) = json_obj.get("header") {
                    //check types here TODO
                    fields.iter().for_each(|field| {
                        if let Some(data) = field_map.get_mut(*field) {
                            if let Some(value) = header.get(*field) {
                                if let Err(e) = data.add_value(value) {
                                    eprintln!("Error processing value: {}", e);
                                }
                            }
                        }
                    });
                }
            }
            Dataset::Transactions => {
                if let Some(tx_list) = json_obj.get("transactions") {
                    //check types here TODO
                    fields.iter().for_each(|field| {
                        if let Some(data) = field_map.get_mut(*field) {
                            for tx in tx_list.as_array().unwrap() {
                                if let Some(value) = tx.get(*field) {
                                    if let Err(e) = data.add_value(value) {
                                        eprintln!("Error processing value: {}", e);
                                    }
                                }
                            }
                        }
                    });
                }
            }
            Dataset::Logs => {
                if let Some(log_list) = json_obj.get("logs") {
                    //check types here TODO
                    fields.iter().for_each(|field| {
                        if let Some(data) = field_map.get_mut(*field) {
                            for log in log_list.as_array().unwrap() {
                                if let Some(value) = log.get(*field) {
                                    if let Err(e) = data.add_value(value) {
                                        eprintln!("Error processing value: {}", e);
                                    }
                                }
                            }
                        }
                    });
                }
            } // _ => panic!("Dataset not found"),
        }
    }

    Ok(field_map)
}
