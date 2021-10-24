use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use std::error::Error;
use std::sync::Arc;

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let compiled = compile(Solc::new("./contracts/**/*.sol")).await?;
    let solution_contract = compiled
        .get("Solution")
        .expect("could not find solution contract");
    let factory = ContractFactory::new(
        solution_contract.abi.clone(),
        solution_contract.bytecode.clone(),
        client.clone(),
    );
    println!("Deploying solution contract");
    let mut deployer = factory.deploy(level.instance.parse::<Address>()?)?;
    deployer.tx.set_gas(1_000_000);
    let solution_contract = deployer.legacy().send().await?;
    let solution_address = solution_contract.address();
    println!("Solution contract deployed at {}\n", solution_address);

    Ok(())
}
