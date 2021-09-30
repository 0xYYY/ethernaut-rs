use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ethernaut {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct Level {
    #[serde(default)]
    pub index: usize,
    pub name: String,
    pub address: String,
    pub instance: String,
    pub completed: bool,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct LevelsConfig {
    pub ethernaut: Ethernaut,
    pub levels: Vec<Level>,
}

#[derive(Serialize, Deserialize)]
pub struct Network {
    pub rpc: String,
    pub chain_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub network: Network,
    pub wallet_mnemonic_path: String,
}
