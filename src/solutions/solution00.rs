use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

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
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

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
        .await?;
    println!(
        "authenticate():\n{}",
        serde_json::to_string_pretty(&receipt)?
    );

    Ok(())
}
