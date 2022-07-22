use std::fs::{File};
use env_logger;
use futures::executor::block_on;
use indy_vdr::pool::helpers::perform_refresh;
use indy_vdr::pool::{Pool, PoolBuilder, PoolTransactions};
use clap::Parser;
use reqwest::blocking;
use log::{info};
use std::{io};
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    /// Path to the pool transactions genesis file
    #[clap(short, long, value_parser, default_value="./pool_transactions_genesis")]
    genesis_file: String,
    /// Path to the output file
    #[clap(short, long, value_parser, default_value="./pool_transactions_genesis")]
    output_file: String,
    /// Always write output file (otherwise the output file is only written if updates were found)
    #[clap(short, long, value_parser, takes_value = false)]
    update_write: bool,
}

fn main() {
    env_logger::init();
    let cfg: Config = Config::parse();
    let mut genesis_path = cfg.genesis_file;
    let out_path = cfg.output_file;
    let updated_write = cfg.update_write;

    // Get genesis file via http if genesis_file path is a url
    if genesis_path.starts_with("http") {
        let default_path = "./pool_transactions_genesis";
        let mut resp = blocking::get(genesis_path).unwrap();
        let mut out = File::create(default_path).expect("failed to create file");
        io::copy(&mut resp, &mut out).expect("failed to copy genesis content to local file");
        genesis_path = default_path.to_string();
    }

    let genesis_txns = PoolTransactions::from_json_file(genesis_path).expect("Could not parse genesis file");
    // Initialize pool
    let pool_builder = PoolBuilder::default().transactions(genesis_txns.clone()).unwrap();
    let pool = pool_builder.into_shared().unwrap();

    // Refresh pool
    let (txns, _timing) = block_on(perform_refresh(&pool)).unwrap();

    let mut updated = false;
    let pool = if let Some(txns) = txns {
        updated = true;
        let builder = {
            let mut pool_txns = genesis_txns;
            pool_txns.extend_from_json(&txns).unwrap();
            PoolBuilder::default().transactions(pool_txns.clone()).unwrap()
        };
        builder.into_shared().unwrap()
    } else {
        pool
    };

    if updated_write || updated {
        // Get updated json and write to file
        let updated_genesis = pool.get_json_transactions().unwrap();
        let mut output_file = File::create(out_path.to_owned()).expect("Could not create output file");
        for line in updated_genesis {
            output_file.write(line.as_bytes()).expect("Could not write to output file");
            output_file.write("\n".to_string().as_bytes()).expect("Could not write to output file");
        }
        info!("Created new genesis_file: file {}", out_path)
    } else {
        info!("No updates found.")
    }
}
