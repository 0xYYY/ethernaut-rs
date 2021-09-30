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
        function authenticate(string)
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let mnemonic = fs::read_to_string(&config.wallet_mnemonic_path)?;
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .build()?;
    let provider = Provider::<Http>::try_from(config.network.rpc.clone())?;
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let contract = LevelContract::new(level.instance.parse::<Address>()?, client.clone());

    let receipt = contract
        .authenticate(String::from("ethernaut0"))
        .legacy()
        .send()
        .await?
        .await?
        .unwrap();

    println!("Solve TX: {:?}", receipt.transaction_hash.to_string());

    Ok(())
}
