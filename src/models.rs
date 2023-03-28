use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    pub addr: u64,
    pub value: Val
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
        Self { tag: 0, value: "nil".to_string() }
    }
    
}

pub static NODES: [&str;2] = ["3030","3031"];
pub static NUM_NODES: u8 = 3;