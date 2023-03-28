// use chrono::{DateTime, Utc};
use tokio;
use tokio::time::Duration;
use crate::db::Peers;
use std::ops::Deref;

pub async fn main_worker(peers: Peers) {
    let client = reqwest::Client::new();
    let mut peer_ports = peers.lock().await;
    
    loop {
        tokio::time::sleep(Duration::from_millis(2000)).await;
        let peers = peer_ports.deref().clone();
        for p in peers {
            let uri = format!("{}{}{}","http://127.0.0.1:",p,"/status/");
            let resp = client.get(uri).send().await;
            match resp {
                Ok(response) => {
                    println!("peer {} is ok! {:?}",p,response);
                },
                Err(_e) => {
                    println!("PEER {} DOWN, removing from peer list", p);
                    peer_ports.retain(|&x| x != p);
                    println!("new peer list {:?}", &peer_ports)
                }
            }
        }
    }
}