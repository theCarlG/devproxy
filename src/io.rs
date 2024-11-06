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

#[cfg(test)]
mod test {
    use std::iter::repeat_with;
    use std::time::{Duration, SystemTime};

    use crate::error;
    use crate::io::{random_fail, throttled_copy};

    use tokio::io::{BufReader, BufWriter};

    #[tokio::test]
    async fn test_random_fail() {
        let result = random_fail(1.0)
            .await
            .map_err(|err| err.downcast::<error::Error>().unwrap());

        assert!(result.is_err());
        assert_eq!(result, Err(error::Error::ExpectedTransferFail));
    }

    #[tokio::test]
    async fn test_throttled_copy() {
        let mut rng = fastrand::Rng::new();

        let bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(10_000).collect();
        let mut reader = BufReader::new(bytes.as_slice());
        let mut writer = BufWriter::new(Vec::new());

        let start = SystemTime::now();
        let result = throttled_copy(&mut reader, &mut writer, 10000).await;
        let end = SystemTime::now();

        let duration = end.duration_since(start).unwrap();

        assert!(result.is_ok());
        assert!(duration >= Duration::from_secs(1));
    }
}
