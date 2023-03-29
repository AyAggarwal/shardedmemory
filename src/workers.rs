use crate::db::Peers;
use std::ops::Deref;
use tokio::time::Duration;

pub async fn main_worker(peers: Peers) {
    let client = reqwest::Client::new();
    //grace period to start all nodes
    tokio::time::sleep(Duration::from_millis(20000)).await;
    loop {
        tokio::time::sleep(Duration::from_millis(40000)).await;
        {
            let mut peer_ports = peers.lock().await;
            let peers = peer_ports.deref().clone();
            for p in peers {
                let uri = format!("{}{}{}", "http://127.0.0.1:", p, "/status/");
                let resp = client.get(uri).send().await;
                match resp {
                    Ok(_response) => {
                        println!("peer {} is ok!", p);
                    }
                    Err(_e) => {
                        println!("PEER {} DOWN, removing from peer list", p);
                        peer_ports.retain(|&x| x != p);
                        println!("new peer list {:?}", &peer_ports)
                    }
                }
            }
        }
    }
}
