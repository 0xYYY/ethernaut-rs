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
        function info() returns (string)
        function info1() returns (string)
        function info2(string) returns (string)
        function infoNum() returns (uint)
        function info42() returns (string)
        function theMethodName() returns (string)
        function method7123949() returns (string)
        function password() returns (string)
        function authenticate(string)
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

    println!("info(): {}", contract.info().call().await?);
    println!("info1(): {}", contract.info_1().call().await?);
    println!(
        "info2(\"hello\"): {}",
        contract.info_2("hello".into()).call().await?
    );
    println!("infoNum(): {}", contract.info_num().call().await?);
    println!("info42(): {}", contract.info_42().call().await?);
    println!(
        "theMethodName(): {}",
        contract.the_method_name().call().await?
    );
    println!(
        "method7123949(): {}",
        contract.method_7123949().call().await?
    );
    let password = contract.password().call().await?;
    println!("password(): {}", password);
    let receipt = contract
        .authenticate(password)
        .legacy()
        .send()
        .await?
        .await?
        .unwrap();
    println!("Solve TX: {:?}", receipt.transaction_hash.to_string());

    Ok(())
}
