use std::net::SocketAddr;

use color_eyre::{self, eyre};
use prometheus_exporter_base::prelude::Authorization;
use prometheus_exporter_base::prelude::ServerOptions;
use prometheus_exporter_base::render_prometheus;
use prometheus_wasd_exporter::{config, serve_metrics};
#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let channels: Vec<String> = config::read_conf()?;
    let addr: SocketAddr = ([0, 0, 0, 0], 9420).into();
    let server_options = ServerOptions {
        addr,
        authorization: Authorization::None,
    };
    let serve = |_, _| async move { Ok(serve_metrics(&channels.clone()).await) };
    render_prometheus(server_options, (), serve).await;
    Ok(())
}
