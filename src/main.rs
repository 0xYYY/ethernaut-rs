mod solutions;
mod types;
use crate::solutions::*;
use crate::types::*;

use clap::{App, Arg};
use ethers::prelude::*;
use ethers_core::types::Address;
use ethers_providers::{Http, Provider};
use ethers_signers::{coins_bip39::English, MnemonicBuilder};
use serde_json;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{convert::TryFrom, sync::Arc};

fn read_levels_config<P: AsRef<Path>>(path: P) -> Result<LevelsConfig, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut c: LevelsConfig = serde_json::from_reader(reader)?;
    (0..c.levels.len()).for_each(|i| c.levels[i].index = i);
    Ok(c)
}

fn update_levels_config(config: &mut LevelsConfig) {
    fs::write(
        "./levels.json",
        serde_json::to_string_pretty(&config).unwrap(),
    )
    .expect("Unable to write file");
}

fn read_environment_config<P: AsRef<Path>>(path: P) -> Result<EnvironmentConfig, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let c = serde_json::from_reader(reader)?;
    Ok(c)
}

fn status(config: &LevelsConfig) {
    config.levels.iter().for_each(|l| {
        info(l, false);
    });
}

fn info(level: &Level, verbose: bool) {
    println!(
        "[{}] Level {:02}: {}",
        if level.instance.is_empty() {
            "."
        } else if !level.completed {
            "~"
        } else {
            "v"
        },
        level.index,
        level.name,
    );
    if verbose {
        if !level.instance.is_empty() {
            println!("Instance address: {}", level.instance);
        }
        println!("{}", level.description);
    }
}

abigen!(
    EthernautContract,
    "contracts/build/Ethernaut.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);

/* fn create_ethernaut_contract(
    ethernaut: &Ethernaut,
    config: &EnvironmentConfig,
) -> Result<Contract<dyn Middleware<>>, Box<dyn Error>> {
    let mnemonic = fs::read_to_string(&config.wallet_mnemonic_path)?;
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .build()?;
    let provider = Provider::<Http>::try_from(config.network.rpc.clone())?;
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let contract_address: Address = ethernaut.address.parse()?;
    let contract = EthernautContract::new(contract_address, client.clone());
    Ok(contract)
} */

async fn new(
    ethernaut: &Ethernaut,
    level: &Level,
    config: &EnvironmentConfig,
) -> Result<Address, Box<dyn Error>> {
    let mnemonic = fs::read_to_string(&config.wallet_mnemonic_path)?;
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str().trim())
        .build()?
        .with_chain_id(config.network.chain_id);
    let provider = Provider::<Http>::try_from(config.network.rpc.clone())?;
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let contract_address: Address = ethernaut.address.parse()?;
    let contract = EthernautContract::new(contract_address, client.clone());

    let receipt = contract
        .create_level_instance(level.address.parse()?)
        .legacy()
        .send()
        .await?
        .await?
        .unwrap();

    println!(
        "create_level_instance():\n{}\n",
        serde_json::to_string_pretty(&receipt)?
    );

    let event: (Address, Address) = contract
        .decode_event(
            "LevelInstanceCreatedLog",
            receipt.logs[0].topics.clone(),
            receipt.logs[0].data.clone(),
        )
        .unwrap();

    println!("Level instance address: {:?}", event.1);

    Ok(event.1)
}

async fn submit(
    ethernaut: &Ethernaut,
    level: &Level,
    config: &EnvironmentConfig,
) -> Result<bool, Box<dyn Error>> {
    let mnemonic = fs::read_to_string(&config.wallet_mnemonic_path)?;
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str().trim())
        .build()?
        .with_chain_id(config.network.chain_id);
    let provider = Provider::<Http>::try_from(config.network.rpc.clone())?;
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let contract_address: Address = ethernaut.address.parse()?;
    let contract = EthernautContract::new(contract_address, client.clone());

    let receipt = contract
        .submit_level_instance(level.instance.parse()?)
        .legacy()
        .send()
        .await?
        .await?
        .unwrap();

    println!(
        "submit_level_instance():\n{}\n",
        serde_json::to_string_pretty(&receipt)?
    );

    let completed = receipt.logs.len() > 0;

    println!("Level completed: {}", completed);

    Ok(completed)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read config of levels
    let mut levels_config = read_levels_config("./levels.json").unwrap();

    // Read config of environment
    let environment_config = read_environment_config("./environment.json").unwrap();

    let level_validator = |v: &str| -> Result<(), String> {
        let err_msg = String::from(format!("range is 0~{}", levels_config.levels.len() - 1));
        match v.parse::<usize>() {
            Ok(v) => {
                if v < levels_config.levels.len() {
                    Ok(())
                } else {
                    Err(err_msg)
                }
            }
            Err(_) => Err(err_msg),
        }
    };
    let matches = App::new("Ethernaut challenges in Rust")
        .version("0.1")
        .author("0xYYY")
        .about("A command line tool to view, solve and submit Ethernaut challenges.")
        .subcommand(App::new("status").about("Print status of each level"))
        .subcommand(
            App::new("info").about("Print level info").arg(
                Arg::new("LEVEL")
                    .about("level index, e.g. 13")
                    .required(true)
                    .index(1)
                    .validator(level_validator),
            ),
        )
        .subcommand(
            App::new("new")
                .about("Create a new instance of a level")
                .arg(
                    Arg::new("LEVEL")
                        .about("level index, e.g. 13")
                        .required(true)
                        .index(1)
                        .validator(level_validator),
                ),
        )
        /* .subcommand(
            App::new("test")
                .about("Locally test the solution to a level")
                .arg(
                    Arg::new("LEVEL")
                        .about("level index, e.g. 13")
                        .required(true)
                        .index(1)
                        .validator(level_validator),
                ),
        ) */
        .subcommand(
            App::new("solve").about("Run solution").arg(
                Arg::new("LEVEL")
                    .about("level index, e.g. 13")
                    .required(true)
                    .index(1)
                    .validator(level_validator),
            ),
        )
        .subcommand(
            App::new("submit")
                .about("Submit to check whether the level is solved")
                .arg(
                    Arg::new("LEVEL")
                        .about("level index, e.g. 13")
                        .required(true)
                        .index(1)
                        .validator(level_validator),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("status", _)) => status(&levels_config),
        Some((command, matches)) => {
            let ethernaut = &levels_config.ethernaut;
            let level_index = matches.value_of("LEVEL").unwrap().parse::<usize>().unwrap();
            let level = &levels_config.levels[level_index];
            match command {
                "info" => info(level, true),
                "new" => {
                    let instance_address = new(ethernaut, level, &environment_config).await?;
                    levels_config.levels[level_index].instance = format!("{:?}", instance_address);
                    update_levels_config(&mut levels_config);
                }
                "solve" => match level_index {
                    0 => solution00::solve(level, &environment_config).await?,
                    1 => solution01::solve(level, &environment_config).await?,
                    _ => {}
                },
                "submit" => {
                    let completed = submit(ethernaut, level, &environment_config).await?;
                    levels_config.levels[level_index].completed = completed;
                    update_levels_config(&mut levels_config);
                }
                _ => {}
            }
        }
        None => {}
    }

    Ok(())
}
