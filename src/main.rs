use std::net::SocketAddr;

use color_eyre::{self, eyre};
use prometheus_exporter_base::prelude::Authorization;
use prometheus_exporter_base::prelude::ServerOptions;
use prometheus_exporter_base::render_prometheus;
use prometheus_wasd_exporter::{config, serve_metrics};
use tokio::signal::unix::signal;
use tokio::signal::unix::SignalKind;
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
    let mut sighup = signal(SignalKind::hangup())?;
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::select! {
        res = tokio::spawn(render_prometheus(server_options, (), serve)) => res?,
        _ = sighup.recv() => todo!(),
        _ = sigint.recv() => println!("recieved SIGINT, exiting..."),
        _ = sigterm.recv() => println!("recieve SIGTERM, exiting...")
    }
    Ok(())
}
