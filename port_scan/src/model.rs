use std::sync::Arc;

#[derive(Debug)]
pub struct PortScanResult {
    pub domain: Arc<String>,
    pub ports: Vec<Port>,
}

#[derive(Debug)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}
