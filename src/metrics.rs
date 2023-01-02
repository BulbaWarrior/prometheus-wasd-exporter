use super::wasd::ChannelStats;
use prometheus_exporter_base::prelude::*;

macro_rules! metrics_for {
    ($metrics:ident, $channels:ident, $($metric:ident $(as $type:ident)?),* ) => {
        $(
            let mut metric: PrometheusMetric = PrometheusMetric::build()
            .with_metric_type(MetricType::Gauge)
            .with_name(stringify!($metric))
            .with_help("Autogenerated")
            .build();
            for stats in &$channels {
                let instance = PrometheusInstance::new()
                    .with_label("channel", &*stats.channel_name)
                    .with_value(stats.$metric $(as $type)?);
                metric.render_and_append_instance(&instance);
            }
            $metrics.push(metric);
          )*
    };
}

pub(crate) fn generate_metrics(channels: Vec<ChannelStats>) -> String {
    let mut metrics = vec![];
    metrics_for!(
        metrics,
        channels,
        channel_clips_count,
        channel_id,
        channel_is_live as u8,
        channel_priority,
        followers_count,
        is_partner as u8
    );
    metrics
        .into_iter()
        .map(|x| x.render())
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::ChannelStats;

    use super::generate_metrics;

    #[test]
    fn test_metrics() {
        let stats = vec![
            ChannelStats {
                channel_clips_count: 0,
                channel_id: 123,
                channel_is_live: true,
                channel_priority: 0.0,
                followers_count: 100,
                is_partner: false,
                channel_name: "aboba".into(),
            },
            ChannelStats {
                channel_clips_count: 0,
                channel_id: 123,
                channel_is_live: false,
                channel_priority: 0.0,
                followers_count: 100,
                is_partner: false,
                channel_name: "abiba".into(),
            },
        ];
        let metrics = generate_metrics(stats);
        println!("{metrics}");
        let res = r#"
# HELP channel_clips_count Autogenerated
# TYPE channel_clips_count gauge
channel_clips_count{channel="aboba"} 0
channel_clips_count{channel="abiba"} 0
# HELP channel_id Autogenerated
# TYPE channel_id gauge
channel_id{channel="aboba"} 123
channel_id{channel="abiba"} 123
# HELP channel_is_live Autogenerated
# TYPE channel_is_live gauge
channel_is_live{channel="aboba"} 1
channel_is_live{channel="abiba"} 0
# HELP channel_priority Autogenerated
# TYPE channel_priority gauge
channel_priority{channel="aboba"} 0
channel_priority{channel="abiba"} 0
# HELP followers_count Autogenerated
# TYPE followers_count gauge
followers_count{channel="aboba"} 100
followers_count{channel="abiba"} 100
# HELP is_partner Autogenerated
# TYPE is_partner gauge
is_partner{channel="aboba"} 0
is_partner{channel="abiba"} 0
"#;
        assert_eq!(metrics.trim(), res.trim())
    }
}
