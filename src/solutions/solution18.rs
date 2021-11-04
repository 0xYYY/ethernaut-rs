use crate::types::*;
use crate::utils;
use ethers;
use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function setSolver(address) public
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    /* EVM opcode resource:
     * https://blog.openzeppelin.com/deconstructing-a-solidity-contract-part-i-introduction-832efd2d7737/
     * https://ethervm.io/
     */

    /*
     * // Part 1: deployment code
     * [PUSH1]    0x60 0x0A (runtime code length, in bytes)
     * [PUSH1]    0x60 0x0C (runtime code offset in calldata)
     * [PUSH1]    0x60 0x00 (offset in memory to store the runtime code)
     * [CODECOPY] 0x39      (memory[destOffset:destOffset+length] = calldata[offset:offset+length])
     * [PUSH1]    0x60 0x0A (return value length, in bytes)
     * [PUSH1]    0x60 0x00 (return value offset in memory)
     * [RETURN]   0xF3      (return value at memory[offset:offset+length])
     * // Part 2: runtime code
     * [PUSH1]    0x60 0x2A (return value)
     * [PUSH1]    0x60 0x00 (offset in memory to store the return value)
     * [MSTORE]   0x52      (store 42 at memory 0)
     * [PUSH1]    0x60 0x20 (return value length, in bytes)
     * [PUSH1]    0x60 0x00 (return value offset in memory)
     * [RETURN]   0xF3      (return value at memory[offset:offset+length])
     */

    let data: Vec<u8> = vec![
        0x60, 0x0A, 0x60, 0x0C, 0x60, 0x00, 0x39, 0x60, 0x0A, 0x60, 0x00, 0xF3, 0x60, 0x2A, 0x60,
        0x00, 0x52, 0x60, 0x20, 0x60, 0x00, 0xF3,
    ];
    // To send a contract creation transaction, leave the `to` field empty
    let tx = TransactionRequest::new().data(data);
    let receipt = client.send_transaction(tx, None).await?.await?;
    println!("Deploy:\n{}\n", serde_json::to_string_pretty(&receipt)?);

    let contract = LevelContract::new(level.instance.parse::<Address>()?, client.clone());
    let receipt = contract
        .set_solver(receipt.unwrap().contract_address.unwrap())
        .legacy()
        .send()
        .await?
        .await?;
    println!("solve():\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
