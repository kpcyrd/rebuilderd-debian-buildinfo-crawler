use crate::errors::*;
use reqwest::Client;
use std::fs;

pub async fn fetch_http(client: &Client, url: &str) -> Result<Vec<u8>> {
    info!("Downloading from {:?}", url);
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send request")?;

    debug!("Received http response: {}", response.status());
    let content = response
        .error_for_status()
        .context("Server responded with error")?
        .bytes()
        .await
        .context("Failed to download content")?;

    debug!("Downloaded {} bytes", content.len());

    Ok(content.to_vec())
}

pub async fn read_path_or_url(client: &Client, s: &str) -> Result<Vec<u8>> {
    if s.starts_with('/') || s.starts_with('.') {
        info!("Reading from disk {:?}", s);
        let content =
            fs::read(&s).with_context(|| anyhow!("Failed to read file from disk: {:?}", s))?;
        debug!("Read {} bytes", content.len());
        Ok(content)
    } else {
        fetch_http(client, s).await
    }
}
