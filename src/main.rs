#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
use clap::Parser;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    time,
};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

use std::{env, net::SocketAddrV4, time::Duration};

mod error;
mod io;

const DEFAULT_SPEED: usize = 10000;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Latency in ms
    #[arg(short, long, env = "LATENCY")]
    latency: Option<u64>,

    /// The network speed in Kib/s
    #[arg(short, long, default_value_t=DEFAULT_SPEED, env="SPEED")]
    speed: usize,

    /// Connect failure rate
    #[arg(short, long, env = "CONNECT_FAILURE_RATE")]
    connect_failure_rate: Option<f32>,

    /// Data transfer failure rate
    #[arg(short, long, env = "TRANSFER_FAILURE_RATE")]
    transfer_failure_rate: Option<f32>,

    #[arg(env = "LISTEN")]
    listen: SocketAddrV4,

    #[arg(env = "ENDPOINT")]
    endpoint: SocketAddrV4,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if env::var("RUST_LOG").is_err() {
        #[cfg(debug_assertions)]
        #[allow(unsafe_code)]
        unsafe {
            env::set_var("RUST_LOG", "devproxy=debug");
        };

        #[cfg(not(debug_assertions))]
        env::set_var("RUST_LOG", "devproxy=info");
    }

    let stdout = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_target(false)
        .with_line_number(false)
        .with_file(false)
        .without_time()
        .with_filter(EnvFilter::from_default_env());

    tracing_subscriber::registry().with(stdout).init();

    let commit = env!("GIT_SHA");
    let version = env!("CARGO_PKG_VERSION");
    let build_date = env!("BUILD_DATE");

    tracing::info!(commit, version, build_date, "starting");
    tracing::info!("listening on: {}", cli.listen);
    tracing::info!("proxying to: {}", cli.endpoint);

    if let Some(failure_rate) = cli.connect_failure_rate {
        tracing::info!("{}% connect failure rate", failure_rate * 100.);
    }
    if let Some(failure_rate) = cli.transfer_failure_rate {
        tracing::info!("{}% transfer failure rate", failure_rate * 100.);
    }

    if let Some(rtt) = cli.latency {
        tracing::info!("{rtt} ms latency");
    }

    let max_bytes_sec = cli.speed * 1024;
    tracing::info!(
        "speed trottled to {}/s",
        humansize::format_size(max_bytes_sec, humansize::BINARY)
    );
    let latency = cli.latency.map_or(Duration::ZERO, Duration::from_millis);

    let listener = TcpListener::bind(cli.listen).await?;
    while let Ok((mut inbound, peer)) = listener.accept().await {
        if latency > Duration::ZERO {
            time::sleep(latency).await;
        }

        if let Some(failure_rate) = cli.connect_failure_rate {
            if fastrand::f32() < failure_rate {
                tracing::info!("{peer}: connection reset due to hitting connection failure rate.",);
                inbound.shutdown().await.ok();
                continue;
            }
        }

        match TcpStream::connect(cli.endpoint).await {
            Ok(mut outbound) => {
                tokio::spawn(async move {
                    io::bidirectional_throttled_copy(
                        &mut inbound,
                        &mut outbound,
                        max_bytes_sec,
                        cli.transfer_failure_rate,
                    )
                    .await
                    .map_err(|err| {
                        if matches!(err.downcast_ref(), Some(error::Error::ExpectedTransferFail)) {
                            tracing::info!(
                                "{peer}: transfer failure due to hitting transfer failure rate"
                            );
                        } else {
                            tracing::error!("failed to transfer; error={err}");
                        }
                    })
                });
            }
            Err(err) => {
                tracing::error!("failed to reach endpoint; error={err}");
                inbound.shutdown().await.ok();
            }
        };
    }

    Ok(())
}
