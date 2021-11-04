use crate::types::*;
use crate::utils;
use ethers;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use std::error::Error;
use std::sync::Arc;

abigen!(
    SolutionContract,
    r#"[
        function solve() public
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    /*
     * Utilize hevm from DappTools to obtain how many gas are used until the GAS opcode (gasleft())
     *
     * hevm exec --code `seth code LEVEL_INSTANCE_ADDRESS`
     * --rpc-url=RPC_URL # e.g. Alchemy RPC
     * --caller 0x0000000000000000000000000000000000000001
     * --origin 0x0000000000000000000000000000000000000002
     * --calldata `seth calldata "enter(bytes8)" 0x0000000000000000`
     * --gas 1000000
     * --debug
     *
     * Step forward until GAS is executed, and we can see that "Gas available" is 999,746, which
     * means gas used is `1,000,000 - 999,746 = 254`. Use this knowledge to set `gas` in the
     * solution contract when calling `enter()`
     */

    let compiled = compile(Solc::new("./contracts/13_gatekeeper_one/solution.sol")).await?;
    let solution_contract = compiled
        .get("Solution")
        .expect("could not find solution contract");
    let factory = ContractFactory::new(
        solution_contract.abi.clone(),
        solution_contract.bytecode.clone(),
        client.clone(),
    );
    println!("Deploying solution contract");
    let contract = factory
        .deploy(level.instance.parse::<Address>()?)?
        .legacy()
        .send()
        .await?;
    let addr = contract.address();
    println!("Solution contract deployed at {}\n", addr);

    let contract = SolutionContract::new(addr, client.clone());
    let receipt = contract.solve().legacy().send().await?.await?;
    println!("solve():\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
