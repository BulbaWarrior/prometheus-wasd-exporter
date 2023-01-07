use color_eyre::eyre::{self, eyre};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Default)]
struct StreamStats {
    stream_total_viewers: u64,
    stream_current_viewers: u64,
    stream_current_active_viewers: u64,
}

#[derive(Deserialize, Debug)]
struct ChannelStats {
    channel_clips_count: u64,
    channel_id: u64,
    channel_is_live: bool,
    channel_priority: f64,
    followers_count: u64,
    is_partner: bool,
    channel_name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Stats {
    pub(crate) stream_total_viewers: u64,
    pub(crate) stream_current_viewers: u64,
    pub(crate) stream_current_active_viewers: u64,
    pub(crate) channel_clips_count: u64,
    pub(crate) channel_id: u64,
    pub(crate) channel_is_live: bool,
    pub(crate) channel_priority: f64,
    pub(crate) followers_count: u64,
    pub(crate) is_partner: bool,
    pub(crate) channel_name: String,
}

impl From<(ChannelStats, StreamStats)> for Stats {
    fn from((channel, stream): (ChannelStats, StreamStats)) -> Self {
        Stats {
            channel_clips_count: channel.channel_clips_count,
            channel_id: channel.channel_id,
            channel_is_live: channel.channel_is_live,
            channel_priority: channel.channel_priority,
            followers_count: channel.followers_count,
            is_partner: channel.is_partner,
            channel_name: channel.channel_name,
            stream_total_viewers: stream.stream_total_viewers,
            stream_current_viewers: stream.stream_current_viewers,
            stream_current_active_viewers: stream.stream_current_active_viewers,
        }
    }
}

pub(crate) async fn get_stats(channel: &str) -> eyre::Result<Stats> {
    let url = format!("https://wasd.tv/api/v2/broadcasts/public?channel_name={channel}");
    let body = reqwest::get(url).await?;
    let json: Value = body.json().await?;
    let channel_stats_json: Option<&Value> = (|| json.get("result")?.get("channel"))();
    let channel_stats: ChannelStats = match channel_stats_json {
        Some(x) => serde_json::from_value(x.to_owned())?,
        None => {
            return Err(eyre!(
                "bad data from wasd api for channel {channel}, got json:\n{json:#}"
            ))
        }
    };
    let stream_stats_json: Option<&Value> = (|| {
        json.get("result")?
            .get("media_container")?
            .get("media_container_streams")?
            .get(0)
    })();
    let stream_stats = match stream_stats_json {
        Some(x) => serde_json::from_value(x.to_owned())?,
        None => StreamStats::default(),
    };
    let stats = (channel_stats, stream_stats).into();
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::get_stats;

    #[tokio::test]
    async fn test_get_stats() {
        let stats = get_stats("dmitron1").await.unwrap();
        dbg!(&stats);
    }
}
