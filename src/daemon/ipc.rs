use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::error::PrismResult;

pub struct IpcServer {
    addr: SocketAddr,
}

impl IpcServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn run(&self) -> PrismResult<()> {
        let listener = TcpListener::bind(self.addr).await?;
        loop {
            let (stream, _) = listener.accept().await?;
            tokio::spawn(async move {
                if let Err(err) = handle_client(stream).await {
                    log::warn!("ipc client error: {err}");
                }
            });
        }
    }
}

async fn handle_client(mut stream: TcpStream) -> PrismResult<()> {
    let mut buffer = vec![0u8; 512];
    let n = stream.read(&mut buffer).await?;
    let command = String::from_utf8_lossy(&buffer[..n]);
    log::info!("ipc received: {command}");
    stream.write_all(b"ok").await?;
    Ok(())
}
