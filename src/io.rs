use tokio::{
    io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    time,
};

use std::time::Duration;

pub async fn throttled_copy<R, W>(
    mut reader: R,
    mut writer: W,
    max_bytes_per_sec: usize,
) -> anyhow::Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let throttle_delay = Duration::from_millis(100);

    let mut buffer = vec![0u8; max_bytes_per_sec / 10];

    loop {
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            break; // EOF reached
        }

        writer.write_all(&buffer[..n]).await?;

        time::sleep(throttle_delay).await;
    }

    writer.flush().await?;
    Ok(())
}

async fn random_fail(failure_rate: f32) -> anyhow::Result<()> {
    loop {
        if fastrand::f32() < failure_rate {
            anyhow::bail!(crate::error::Error::ExpectedTransferFail);
        }
        time::sleep(Duration::from_secs(1)).await;
    }
}

pub async fn bidirectional_throttled_copy<S>(
    stream_a: S,
    stream_b: S,
    max_bytes_per_sec: usize,
    transfer_failure_rate: Option<f32>,
) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let (a_read, a_write) = io::split(stream_a);
    let (b_read, b_write) = io::split(stream_b);

    let a_to_b = throttled_copy(a_read, b_write, max_bytes_per_sec);
    let b_to_a = throttled_copy(b_read, a_write, max_bytes_per_sec);

    if let Some(failure_rate) = transfer_failure_rate {
        tokio::select! {
            result = a_to_b => result,
            result = b_to_a => result,
            result = random_fail(failure_rate)=> result,
        }
    } else {
        tokio::select! {
            result = a_to_b => result,
            result = b_to_a => result,
        }
    }
}
