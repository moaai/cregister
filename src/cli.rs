use clap::{ColorChoice, Parser, Subcommand};
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(color=ColorChoice::Always)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[arg(short = 's', long)]
    server: Ipv4Addr,
    #[arg(short = 'p', long, default_value = "5001")]
    port: u16,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "List cash register items")]
    // FIXME: Should we support multiple commands, like list products and then take device
    // version???
    /*
        Rist {
            /*
            #[arg(short, long, help = "Show products")]
            products: bool,
            #[arg(short, long, help = "Get cash register version")]
            model: bool,
            */
        },
    */
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
    //#[clap(subcommand)]
    //Products,
    Products {
        #[arg(short, long, help = "Start date")]
        start: Option<String>,
        #[arg(short, long, help = "End date")]
        end: Option<String>,
    },
    #[command(about = "Get model version")]
    Model,
}

pub struct Options {
    pub server: Ipv4Addr,
    pub port: u16,
    pub command: Commands,
}

impl Options {
    pub fn from_argc(cli: Cli) -> Self {
        let server = cli.server;
        let port = cli.port;

        /*
        }
            */

        Options {
            server,
            port,
            command: cli.command,
        }
    }
}
