use std::collections::HashSet;

use crate::cli::config::Dataset;
use colored::Colorize;

use super::config::Range;

pub fn print_intro(dataset: Dataset, fields: &[String], range: &Range, options: &Vec<&String>) {
    print_header("'\nConfiguration");
    print_bullet_indent("Dataset", get_dataset_string(dataset), 2);
    print_bullet_indent("Fields", &fields.join(", "), 2);
    print_bullet_indent("Range", &format!("{:?}:{:?}", range.start, range.end), 2);
    let comma_separated: String = options
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(", ");
    print_bullet_indent("Options", comma_separated, 2);

    print_schema(dataset, fields);

    println!();
}

fn print_schema(dataset: Dataset, fields: &[String]) {
    print_header("\nSchema");
    for field in fields {
        let field_type = get_field_type(dataset, field);
        print_bullet_indent(field, field_type, 2);
    }
    //let string_slice: &[String] = &fields;
    // Convert &[String] to Vec<&str>
    //let str_slice_vec: Vec<&str> = string_slice.iter().map(|s| s.as_str()).collect();
    // Take a reference to the slice of string slices
    //let str_slice: &[&str] = &str_slice_vec;
    //get_other_available_fields(dataset, str_slice);
}

fn get_field_type(dataset: Dataset, field: &String) -> &str {
    match dataset {
        Dataset::Blocks => match field.as_str() {
            "blockHash" => "string",
            "baseFeePerGas" => "number",
            "blockNumber" => "number",
            "miner" => "string",
            "parentHash" => "string",
            "timestamp" => "number",
            "transactions" => "array",
            "transactionsRoot" => "string",
            "uncles" => "array",
            "unclesHash" => "string",
            "gasUsed" => "number",
            "gasLimit" => "number",
            "difficulty" => "number",
            "totalDifficulty" => "number",
            "size" => "number",
            "extraData" => "string",
            "logsBloom" => "string",
            "receiptsRoot" => "string",
            "sha3Uncles" => "string",
            "stateRoot" => "string",
            "sealFields" => "array",
            "hash" => "string",
            "number" => "number",
            _ => "unknown",
        },
        Dataset::Transactions => match field.as_str() {
            "blockHash" => "string",
            "blockNumber" => "number",
            "from" => "string",
            "gas" => "number",
            "gasPrice" => "number",
            "hash" => "string",
            "input" => "string",
            "nonce" => "number",
            "to" => "string",
            "transactionIndex" => "number",
            "value" => "number",
            "v" => "string",
            "r" => "string",
            "s" => "string",
            _ => "unknown",
        },
        Dataset::Logs => match field.as_str() {
            "address" => "string",
            "blockHash" => "string",
            "blockNumber" => "number",
            "data" => "string",
            "logIndex" => "number",
            "removed" => "boolean",
            "topics" => "array",
            "transactionHash" => "string",
            "transactionIndex" => "number",
            _ => "unknown",
        },
    }
}

pub fn print_header<A: AsRef<str>>(header: A) {
    let header_str = header.as_ref().white().bold();
    let underline = "─".repeat(header_str.len()).truecolor(255, 255, 255);
    println!("{}", header_str);
    println!("{}", underline);
}

fn get_dataset_string(dataset: Dataset) -> String {
    match dataset {
        Dataset::Blocks => "Blocks".to_string(),
        Dataset::Transactions => "Transactions".to_string(),
        Dataset::Logs => "Logs".to_string(),
    }
}

fn print_bullet_indent<A: AsRef<str>, B: AsRef<str>>(key: A, value: B, indent: usize) {
    let bullet_str = "- ".truecolor(0, 153, 255);
    let key_str = key.as_ref().white().bold();
    let value_str = value.as_ref().truecolor(170, 170, 170);
    let colon_str = ": ".truecolor(0, 153, 255);
    println!(
        "{}{}{}{}{}",
        " ".repeat(indent),
        bullet_str,
        key_str,
        colon_str,
        value_str
    );
}

fn _get_other_available_fields(dataset: Dataset, fields: &[&str]) {
    let available_fields = match dataset {
        Dataset::Blocks => vec![
            "blockHash",
            "blockNumber",
            "miner",
            "parentHash",
            "timestamp",
            "transactions",
            "transactionsRoot",
            "uncles",
            "unclesHash",
            "gasUsed",
            "gasLimit",
            "difficulty",
            "totalDifficulty",
            "size",
            "extraData",
            "logsBloom",
            "receiptsRoot",
            "sha3Uncles",
            "stateRoot",
            "sealFields",
            "hash",
            "number",
        ],
        Dataset::Transactions => vec![
            "from", "hash", "input", "nonce", "to", "value", "v", "r", "s",
        ],
        Dataset::Logs => vec![
            "address",
            "blockHash",
            "blockNumber",
            "data",
            "logIndex",
            "topics",
            "transactionHash",
        ],
    };

    _print_other_available_fields(&available_fields, fields);
}
fn _print_other_available_fields(available_fields: &[&str], fields: &[&str]) {
    let fields_set: HashSet<_> = fields.iter().cloned().collect();
    let other_fields: Vec<&str> = available_fields
        .iter()
        .filter(|&&field| !fields_set.contains(&field))
        .cloned()
        .collect();
    let comma_separated = other_fields.join(", ");
    println!("\nOther available fields: {}", comma_separated);
}
