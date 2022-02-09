mod common_ports;
pub mod model;

use futures::{stream, StreamExt};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::{lookup_host, TcpStream};

use common_ports::MOST_COMMON_PORTS;
use model::Port;

pub async fn scan_ports(domain: Arc<String>) -> (Arc<String>, Vec<Port>) {
    let ports = stream::iter(MOST_COMMON_PORTS.into_iter())
        .map(|port| scan_port(domain.clone(), *port))
        .buffer_unordered(100)
        .collect()
        .await;

    (domain, ports)
}

async fn scan_port(domain: Arc<String>, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    let socket_address = format!("{}:{}", *domain, port);
    let socket_address: Vec<SocketAddr> = lookup_host(socket_address)
        .await
        .expect("port scanner: creating socket addr")
        .collect();

    if socket_address.len() == 0 {
        return Port {
            port,
            is_open: false,
        };
    };

    let is_open = if let Ok(_) =
        tokio::time::timeout(timeout, TcpStream::connect(&socket_address[0])).await
    {
        true
    } else {
        false
    };

    Port { port, is_open }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
