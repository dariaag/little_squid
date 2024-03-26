pub fn normalize_progess(start_block: u64, end_block: u64, current_block: u64) -> u64 {
    let total_blocks = end_block - start_block;
    let current_progress = current_block - start_block;
    let normalized_progress = (current_progress * 100) / total_blocks;
    normalized_progress
}
pub fn get_percentage(current_num_blocks: u64, total_num_blocks: u64) -> u64 {
    let percentage = (current_num_blocks * 100) / total_num_blocks;
    percentage
}
pub fn hex_str_to_u64(hex_str: &str) -> Result<u64, std::num::ParseIntError> {
    let trimmed_hex_str = hex_str.trim_start_matches("0x");
    u64::from_str_radix(trimmed_hex_str, 16)
}

pub fn sizeof_val(v: &serde_json::Value) -> usize {
    std::mem::size_of::<serde_json::Value>()
        + match v {
            serde_json::Value::Null => 0,
            serde_json::Value::Bool(_) => 0,
            serde_json::Value::Number(_) => 0, // incorrect if arbitrary_precision is enabled
            serde_json::Value::String(s) => s.capacity(),
            serde_json::Value::Array(a) => a.iter().map(sizeof_val).sum(),
            serde_json::Value::Object(o) => o
                .iter()
                .map(|(k, v)| {
                    std::mem::size_of::<String>()
                        + k.capacity()
                        + sizeof_val(v)
                        + std::mem::size_of::<usize>() * 3 //crude approximation, each map entry has 3 words of overhead
                })
                .sum(),
        }
}
