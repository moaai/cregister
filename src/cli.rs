use clap::{ColorChoice, Parser, Subcommand};
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(color=ColorChoice::Always)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[arg(short = 'd', long)]
    device: Ipv4Addr,
    #[arg(short = 'p', long, default_value = "5001")]
    port: u16,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "List cash register items")]
    // FIXME: Should we support multiple commands, like list products and then take device
    #[command(about = "Send command to the device")]
    Send {
        #[arg(short, long, help = "Send file content")]
        file: PathBuf,
    },
    #[clap(subcommand)]
    Get(ListSubCommand),
}

#[derive(Debug, Subcommand)]
pub enum ListSubCommand {
    #[command(about = "List cash register products")]
    Products {
        #[arg(short, long, help = "Start date")]
        start: Option<String>,
        #[arg(short, long, help = "End date")]
        end: Option<String>,
        #[arg(short, long, help = "Send products from file", default_value = "products.csv")]
        file: PathBuf,
    },
    #[command(about = "Get model version")]
    Model,
}

pub struct Options {
    pub device: Ipv4Addr,
    pub port: u16,
    pub command: Commands,
}

impl Options {
    pub fn from_argc(cli: Cli) -> Self {
        let device = cli.device;
        let port = cli.port;

        Options {
            device,
            port,
            command: cli.command,
        }
    }
}
