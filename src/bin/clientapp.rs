use std::net::SocketAddr;
use std::process;
use std::str::FromStr;

use clap::Parser;

use cregister::client;

use cregister::cli::{Cli, Commands, ListSubCommand, Options};

//List products
//Save products
//Get particualr product???

//TODO connection params

//That fun is a fun ... if addr is en ERR function returns immediately
// fn fun(addr: std::io::Result<()>) -> std::io::Result<()> {
//     let v = addr?;
//     println!("NEVER REACH HERE");
//     Ok(())
// }

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
            ListSubCommand::Products { start, end } => {
                let mut csv_writer = csv::WriterBuilder::new().from_path("products.csv").unwrap();
                client
                    .get_products(start.as_deref(), end.as_deref(), |product| {
                        println!("Got product {}", product);
                        csv_writer.serialize(product).unwrap();
                        //DUMP!(product)
                        // ll_dump(pro, op)
                    })
                    .unwrap();
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

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let mut client = client::Client::connect(SocketAddr::from(([127, 0, 0, 1], 5001))).unwrap();
        // client.get_products(|_p| {}).unwrap();
    }
}
