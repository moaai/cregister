use std::net::SocketAddr;
use std::process;
use std::str::FromStr;

use clap::Parser;

use cregister::cli::{Cli, Commands, ListSubCommand, Options};

use cregister::client;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use log::trace;

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let options = Options::from_argc(cli);

    let mut client = client::Client::connect(
        SocketAddr::from_str(format!("{}:{}", options.device, options.port).as_str())
            .unwrap_or_else(|e| {
                eprintln!("{:?}", e);
                process::exit(1)
            }),
    )
    .unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        process::exit(1);
    });

    match options.command {
        Commands::Get(sc) => match sc {
            ListSubCommand::Products { start, end, file } => {
                let mut csv_writer = csv::WriterBuilder::new()
                    .from_path(file)
                    .expect("Unable to create csv writer");

                let pb = ProgressBar::new_spinner().with_finish(ProgressFinish::AndLeave);
                pb.set_style(
                    ProgressStyle::with_template(
                        "{spinner} [{elapsed_precise}] Downloading products {len:7} {msg}",
                    )
                    .unwrap(),
                );
                client
                    .get_products(start.as_deref(), end.as_deref(), |product| {
                        trace!("{}", product);
                        tick(&pb);
                        // std::thread::sleep(std::time::Duration::from_millis(500));
                        csv_writer
                            .serialize(product)
                            .expect("Failed to save csv file");
                    })
                    .expect("Failed to download products");

                // println!("Completed. {} Products downloaded", prd_cnt);
            }
            ListSubCommand::Model => {
                println!("Model");
            }
        },
        Commands::Send { file } => {
            client.upload_products_from_file(file).unwrap();
        }
    }
}

fn tick(pb: &ProgressBar) {
    if std::env::var("RUST_LOG").is_err() {
        pb.inc(1);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let mut client = client::Client::connect(SocketAddr::from(([127, 0, 0, 1], 5001))).unwrap();
        // client.get_products(|_p| {}).unwrap();
    }
}
