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