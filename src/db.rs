use crate::models::Val;
use crate::models::NODES;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<HashMap<u64, Val>>>;
pub type Peers = Arc<Mutex<Vec<u16>>>;

pub fn blank() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn setup_peers(port: u16) -> Peers {
    let peers_filter = NODES
        .iter()
        .filter(|&&id| id != port)
        .map(|&x| x)
        .collect::<Vec<u16>>();
    Arc::new(Mutex::new(peers_filter))
}
