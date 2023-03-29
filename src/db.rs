use crate::models::VNode;
use crate::models::Val;
use crate::models::NODES;
use hashring::HashRing;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<HashMap<u64, Val>>>;
pub type Peers = Arc<Mutex<Vec<u16>>>;
pub type Me = Arc<Mutex<u16>>;
pub type HashFunction = Arc<Mutex<hashring::HashRing<VNode>>>;

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

pub fn setup_me(port: u16) -> Me {
    Arc::new(Mutex::new(port))
}

pub fn setup_sharding() -> HashFunction {
    let mut hashring: HashRing<VNode> = HashRing::new();
    hashring.add(VNode {
        port1: 3030,
        port2: 3031,
    });
    hashring.add(VNode {
        port1: 3031,
        port2: 3032,
    });
    hashring.add(VNode {
        port1: 3030,
        port2: 3032,
    });
    Arc::new(Mutex::new(hashring))
}
