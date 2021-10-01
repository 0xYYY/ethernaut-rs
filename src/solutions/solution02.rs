use crate::types::*;
use ethers::prelude::*;
use ethers_providers::{Http, Provider};
use ethers_signers::{coins_bip39::English, MnemonicBuilder};
use std::error::Error;
use std::fs;
use std::{convert::TryFrom, sync::Arc};

abigen!(
    LevelContract,
    r#"[
        function Fal1out() public payable
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let mnemonic = fs::read_to_string(&config.wallet_mnemonic_path)?;
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str().trim())
        .build()?
        .with_chain_id(config.network.chain_id);

    let provider = Provider::<Http>::try_from(config.network.rpc.clone())?;
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let contract = LevelContract::new(level.instance.parse::<Address>()?, client.clone());

    let receipt = contract.fal_1out().legacy().send().await?.await?;
    println!("Fal1out():\n{}\n", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
