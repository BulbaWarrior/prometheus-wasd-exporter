use color_eyre::eyre::{self, eyre};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub(crate) struct ChannelStats {
    pub(crate) channel_clips_count: u64,
    pub(crate) channel_id: u64,
    pub(crate) channel_is_live: bool,
    pub(crate) channel_priority: f64,
    pub(crate) followers_count: u64,
    pub(crate) is_partner: bool,
    pub(crate) channel_name: String,
}

pub(crate) async fn get_stats(channel: &str) -> eyre::Result<ChannelStats> {
    let url = format!("https://wasd.tv/api/v2/broadcasts/public?channel_name={channel}");
    let body = reqwest::get(url).await?;
    let json: Value = body.json().await?;
    let stats_json: Option<&Value> = (|| json.get("result")?.get("channel"))();
    let stats: ChannelStats = match stats_json {
        Some(x) => serde_json::from_value(x.to_owned())?,
        None => {
            return Err(eyre!(
                "bad data from wasd api for channel {channel}, got json:\n{json:#}"
            ))
        }
    };
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::get_stats;

    #[tokio::test]
    async fn test_get_stats() {
        let stats = get_stats("Dawgos").await.unwrap();
        dbg!(&stats);
    }
}
