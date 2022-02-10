use anyhow::Result;
use crtsh_data::{CrtShDatasource, Datasource};
use futures::{stream, StreamExt, TryStreamExt};
use port_scan::{
    model::{Port, PortScanResult},
    scan_ports,
};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::Arc,
};
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

impl Opt {
    pub async fn run(&self) -> Result<()> {
        match &self.subcommand {
            Subcommand::SubdomainEnumeration(opt) => {
                let http_client = reqwest::Client::new();
                let datasource = CrtShDatasource::new(http_client);
                let crts = datasource.list_crts(&opt.domain).await?;

                let mut subdomains: HashSet<String> = crts
                    .into_iter()
                    .map(|crt| {
                        crt.name_value
                            .split("\n")
                            .map(|subdomain| subdomain.trim().to_string())
                            .collect::<Vec<String>>()
                    })
                    .flatten()
                    .filter(|subdomain| subdomain != &opt.domain)
                    .filter(|subdomain| !subdomain.contains("*"))
                    .collect();

                subdomains.insert(opt.domain.clone());

                subdomains
                    .into_iter()
                    .for_each(|subdomain| println!("{}", subdomain));
            }
            Subcommand::PortScan(opt) => {
                let file = File::open(&opt.file).expect("Error reading file");
                let reader = BufReader::new(&file);

                let open_ports: Vec<PortScanResult> = stream::iter(reader.lines())
                    .map(|line| {
                        let line = Arc::new(line.unwrap());
                        scan_ports(line.clone())
                    })
                    .buffer_unordered(100)
                    .try_collect::<Vec<PortScanResult>>()
                    .await?
                    .into_iter()
                    .map(|scan| {
                        let open_ports = scan
                            .ports
                            .into_iter()
                            .filter(|port| port.is_open)
                            .collect::<Vec<Port>>();

                        PortScanResult {
                            domain: scan.domain,
                            ports: open_ports,
                        }
                    })
                    .collect();

                open_ports
                    .into_iter()
                    .for_each(|list| println! {"{:#?}", list})
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, StructOpt)]
enum Subcommand {
    SubdomainEnumeration(SubdomainEnumeration),
    PortScan(PortScan),
}

#[derive(Debug, PartialEq, StructOpt)]
struct SubdomainEnumeration {
    domain: String,
}

#[derive(Debug, PartialEq, StructOpt)]
struct PortScan {
    file: PathBuf,
}
