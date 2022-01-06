extern crate web3;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
struct Config {
    web3url: String,
    account: String,
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    let config_file_name = "config.toml";
    let mut config_file_handle = match File::open(&config_file_name) {
        Ok(x) => x,
        Err(e) => {
            println!(
                "Unable to open config file {}, error {:?}.",
                config_file_name, e
            );
            std::process::exit(1);
        }
    };

    let mut config_file_buffer = String::new();
    if let Err(e) = config_file_handle.read_to_string(&mut config_file_buffer) {
        println!(
            "Unable to read config file {}, error {}.",
            config_file_name, e
        );
        std::process::exit(1);
    }

    let cfg: Config = match toml::from_str(&config_file_buffer) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    };

    println!("{:?}", cfg);

    let transport = web3::transports::Http::new(&cfg.web3url)?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push(cfg.account.parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        let bal = balance.as_u64();
        println!("Balance of {:?}: {}", account, bal);
    }

    Ok(())
}
