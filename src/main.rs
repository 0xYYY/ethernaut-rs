mod solutions;
mod types;
mod utils;
use crate::solutions::*;
use crate::types::*;

use clap::{App, Arg};
use ethers::prelude::*;
use ethers_core::types::Address;
use ethers_core::utils::WEI_IN_ETHER;
use serde_json;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

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
    r#"[
        event LevelInstanceCreatedLog(address indexed, address)
        function createLevelInstance(address) public payable
        function submitLevelInstance(address payable) public
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

async fn new(
    ethernaut: &Ethernaut,
    level: &Level,
    config: &EnvironmentConfig,
) -> Result<Address, Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let contract_address: Address = ethernaut.address.parse()?;
    let contract = EthernautContract::new(contract_address, client.clone());

    let receipt = contract
        .create_level_instance(level.address.parse()?)
        .legacy()
        .value(U256::from(level.init_value) * WEI_IN_ETHER)
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
            receipt.logs.last().unwrap().topics.clone(),
            receipt.logs.last().unwrap().data.clone(),
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
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let contract_address: Address = ethernaut.address.parse()?;
    let contract = EthernautContract::new(contract_address, client.clone());

    let receipt = contract
        .submit_level_instance(level.instance.parse()?)
        .legacy()
        .gas(1_000_000)
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
                    levels_config.levels[level_index].completed = false;
                    update_levels_config(&mut levels_config);
                }
                "solve" => match level_index {
                    0 => solution00::solve(level, &environment_config).await?,
                    1 => solution01::solve(level, &environment_config).await?,
                    2 => solution02::solve(level, &environment_config).await?,
                    3 => solution03::solve(level, &environment_config).await?,
                    4 => solution04::solve(level, &environment_config).await?,
                    5 => solution05::solve(level, &environment_config).await?,
                    6 => solution06::solve(level, &environment_config).await?,
                    7 => solution07::solve(level, &environment_config).await?,
                    8 => solution08::solve(level, &environment_config).await?,
                    9 => solution09::solve(level, &environment_config).await?,
                    10 => solution10::solve(level, &environment_config).await?,
                    11 => solution11::solve(level, &environment_config).await?,
                    12 => solution12::solve(level, &environment_config).await?,
                    14 => solution14::solve(level, &environment_config).await?,
                    15 => solution15::solve(level, &environment_config).await?,
                    16 => solution16::solve(level, &environment_config).await?,
                    17 => solution17::solve(level, &environment_config).await?,
                    19 => solution19::solve(level, &environment_config).await?,
                    20 => solution20::solve(level, &environment_config).await?,
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
