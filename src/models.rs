use serde::{Deserialize, Serialize};

pub static NODES: [u16; 3] = [3030, 3031, 3032];

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    pub addr: u64,
    pub value: Val,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Val {
    pub tag: u64,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WriteRequest {
    pub addr: u64,
    pub value: String,
}

impl Default for Val {
    fn default() -> Self {
        Self {
            tag: 0,
            value: "nil".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Hash)]
pub struct VNode {
    pub port1: u16,
    pub port2: u16,
}
