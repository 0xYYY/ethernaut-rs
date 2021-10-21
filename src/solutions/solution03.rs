use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use std::error::Error;
use std::sync::Arc;

abigen!(
    SolutionContract,
    r#"[
        function solve()
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let compiled = compile(Solc::new("./contracts/**/*.sol")).await?;
    let contract = compiled
        .get("Solution")
        .expect("could not find solution contract");
    let factory = ContractFactory::new(
        contract.abi.clone(),
        contract.bytecode.clone(),
        client.clone(),
    );

    println!("Deploying solution contract");
    let contract = factory
        .deploy(level.instance.parse::<Address>()?)?
        .legacy()
        .send()
        .await?;
    let addr = contract.address();
    let contract = SolutionContract::new(addr, client.clone());
    println!("Solution contract deployed at {}\n", addr);

    for i in 0..10 {
        println!("Solve #{}", i);
        let _receipt = contract.solve().legacy().send().await?.await?;
    }

    Ok(())
}
