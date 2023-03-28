use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::models::Val;

pub type Db = Arc<Mutex<HashMap<u64, Val>>>;

pub fn blank() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}