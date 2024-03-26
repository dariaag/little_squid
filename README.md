# Subsquid Network CLI

CLI tool to fetch blockchain data from the Subsquid Network

## Contents

1. [Example Usage](#example-usage)

## Example Usage

test use as `cargo run -- --dataset <DATASET> --range <START:END> --options <OPTIONS>`
or `cargo run -- -d <DATASET> -r <START:END> -o <OPTIONS>`

| Example                                                                          | Command                                                                      |
| :------------------------------------------------------------------------------- | :--------------------------------------------------------------------------- |
| Extract all logs from block 16,000,000 to block 17,000,000                       | `cargo run -- -d logs -r 16000000:17000000`                                  |
| Extract all transactions from block 16,000,000 to block 17,000,000 to an address | `cargo run -- -d transactions -r 16000000:17000000 -o 'to:<ADDRESS>'`        |
| Extract all USDC events                                                          | `cargo run -- -d logs -o address:0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48` |
| Extract all block data                                                           | `cargo run -- -d blocks  `                                                   |

### Options

Blocks do not have options.

| Transactions | Logs      |
| :----------- | :-------- |
| `address`    | `from`    |
| `topic0`     | `to`      |
| `topic1`     | `sighash` |
| `topic2`     |           |
| `topic3`     |           |
