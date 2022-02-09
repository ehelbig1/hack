mod feature;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, PartialEq, StructOpt)]
enum Subcommand {
    Recon(feature::reconnaissance::Opt),
}

impl Subcommand {
    async fn run(&self) -> Result<()> {
        match self {
            Subcommand::Recon(opt) => opt.run().await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    opt.subcommand.run().await
}
