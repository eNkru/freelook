use std::time::Duration;
use tokio::time::timeout;

/// Check network connectivity by making a HEAD request to Outlook
async fn do_check_network() -> bool {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build();

    let client = match client {
        Ok(c) => c,
        Err(_) => return false,
    };

    match timeout(
        Duration::from_secs(15),
        client.head("https://outlook.live.com").send(),
    )
    .await
    {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

pub async fn check_network() -> Result<bool, String> {
    Ok(do_check_network().await)
}

pub async fn reconnect() -> Result<bool, String> {
    Ok(do_check_network().await)
}
