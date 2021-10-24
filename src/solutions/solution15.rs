use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function approve(address, uint256) public virtual returns (bool)
        function balanceOf(address) public view virtual returns (uint256)
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

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
    let solution_contract = compiled
        .get("Solution")
        .expect("could not find solution contract");
    let factory = ContractFactory::new(
        solution_contract.abi.clone(),
        solution_contract.bytecode.clone(),
        client.clone(),
    );
    println!("Deploying solution contract");
    let solution_contract = factory
        .deploy(level.instance.parse::<Address>()?)?
        .legacy()
        .send()
        .await?;
    let solution_address = solution_contract.address();
    println!("Solution contract deployed at {}\n", solution_address);

    let level_contract = LevelContract::new(level.instance.parse::<Address>()?, client.clone());
    let balance = level_contract
        .balance_of(client.default_sender().unwrap())
        .call()
        .await?;
    println!("balnce: {}", balance);
    let receipt = level_contract
        .approve(solution_address, balance)
        .legacy()
        .send()
        .await?
        .await?;
    println!("approve():\n{}", serde_json::to_string_pretty(&receipt)?);

    let solution_contract = SolutionContract::new(solution_address, client.clone());
    let receipt = solution_contract.solve().legacy().send().await?.await?;
    println!("solve():\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
