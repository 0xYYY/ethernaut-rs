use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function Fal1out() public payable
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

    let receipt = contract.fal_1out().legacy().send().await?.await?;
    println!("Fal1out():\n{}\n", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
