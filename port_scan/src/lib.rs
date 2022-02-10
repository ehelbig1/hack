mod common_ports;
mod error;
pub mod model;

use futures::{stream, StreamExt, TryStreamExt};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::{lookup_host, TcpStream};

use common_ports::MOST_COMMON_PORTS;
use error::Error;
use model::{Port, PortScanResult};

pub async fn scan_ports(domain: Arc<String>) -> Result<PortScanResult, Error> {
    let ports = stream::iter(MOST_COMMON_PORTS.into_iter())
        .map(|port| scan_port(domain.clone(), *port))
        .buffer_unordered(100)
        .try_collect()
        .await?;

    Ok(PortScanResult { domain, ports })
}

async fn scan_port(domain: Arc<String>, port: u16) -> Result<Port, Error> {
    let timeout = Duration::from_secs(3);
    let socket_address = format!("{}:{}", *domain, port);
    let socket_address: Vec<SocketAddr> = lookup_host(socket_address).await?.collect();

    if socket_address.len() == 0 {
        return Ok(Port {
            port,
            is_open: false,
        });
    };

    let is_open = if let Ok(_) =
        tokio::time::timeout(timeout, TcpStream::connect(&socket_address[0])).await
    {
        true
    } else {
        false
    };

    Ok(Port { port, is_open })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
