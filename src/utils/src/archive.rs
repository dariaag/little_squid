use reqwest::{self};

pub async fn get_height(archive_url: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}/height", archive_url);
    let body = reqwest::get(&url).await?.text().await?;
    Ok(body)
}

pub async fn get_worker(archive_url: &str, first_block: &str) -> Result<String, reqwest::Error> {
    let url: String = format!("{}/{}/worker", archive_url, first_block);
    let body = reqwest::get(&url).await?.text().await?;
    Ok(body)
}
