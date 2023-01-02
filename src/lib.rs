use futures::stream::{FuturesOrdered, StreamExt};
use wasd::ChannelStats;

pub mod config;
pub mod metrics;
pub mod wasd;

pub async fn serve_metrics(channels: &[String]) -> String {
    let wasd_results = FuturesOrdered::from_iter(channels.into_iter().map(|x| wasd::get_stats(x)));

    let stats: Vec<Option<ChannelStats>> = wasd_results
        .map(|x| match x {
            Ok(x) => Some(x),
            Err(e) => {
                println!("{e}");
                None
            }
        })
        .collect()
        .await;
    let stats: Vec<ChannelStats> = stats
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.expect("impossible unfiltered None"))
        .collect();
    metrics::generate_metrics(stats)
}

#[cfg(test)]
mod tests {
    use crate::serve_metrics;

    #[tokio::test]
    async fn test_serve_metrics() {
        let metrics =
            serve_metrics(&["Dawgos".into(), "Alison".into(), "nonexistent".into()]).await;
        println!("{metrics}");
        assert_eq!(metrics.lines().count(), 24)
    }
}
