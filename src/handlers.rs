use crate::db::Db;
use crate::db::Peers;
use crate::models::Entry;
use crate::models::Val;
use crate::models::WriteRequest;
use std::convert::Infallible;
use std::ops::Deref;
use warp::{self, http::StatusCode};

// reads local value at an address
pub async fn read_address(addr: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    let entries = db.lock().await;
    let value = entries.get(&addr);
    if let Some(x) = value {
        Ok(warp::reply::json(x))
    } else {
        // does not insert to database, reads as nil to user
        // improvement: create rejection flow in routing scheme
        Ok(warp::reply::json(&Val {
            tag: 0,
            value: "nil".to_string(),
        }))
    }
}

// local update function for write-backs
pub async fn update_address(body: Entry, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut memory = db.lock().await;
    memory.insert(body.addr, body.value);
    Ok(StatusCode::OK)
}

// status checker
pub async fn pong() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub async fn read(addr: u64, db: Db, peers: Peers) -> Result<impl warp::Reply, Infallible> {
    //check if I have something with read_address and proceed with 0
    let entries = db.lock().await;
    let value = entries.get(&addr);
    let mut current_tag: u64 = 0;
    let mut current_value = "nil".to_string();
    if let Some(x) = value {
        current_tag = x.tag;
        current_value = x.value.clone();
    }

    //init request client and begin asking for
    let client = reqwest::Client::new();
    let peer_ports = peers.lock().await;
    let peers = peer_ports.deref();
    for p in peers {
        let uri = format!("{}{}{}{}", "http://127.0.0.1:", *p, "/registers/", addr);
        println!("{}", uri);
        let resp = client.get(uri).send().await;
        //we do not care if it returns an error since it will be fail detected
        match resp {
            Ok(response) => {
                println!("reached here!");
                let tryjson = response.json::<Val>().await;
                if let Ok(json) = tryjson {
                    if json.tag > current_tag {
                        println!("json tag is {}", json.tag);
                        current_tag = json.tag;
                        current_value = json.value.clone();
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    if current_tag != 0 {
        Ok(warp::reply::json(&Val {
            tag: current_tag,
            value: current_value,
        }))
    } else {
        Ok(warp::reply::json(&Val {
            tag: 0,
            value: "nil".to_string(),
        }))
    }
}

pub async fn write(
    data: WriteRequest,
    db: Db,
    peers: Peers,
) -> Result<impl warp::Reply, Infallible> {
    //check whatever tag that I have
    let mut entries = db.lock().await;
    let value = entries.get(&data.addr);
    let mut current_tag: u64 = 0;
    if let Some(x) = value {
        current_tag = x.tag;
    }

    //init request client and begin asking for tags
    let client = reqwest::Client::new();

    let peer_ports = peers.lock().await;
    let peers = peer_ports.deref();

    for p in peers {
        let uri = format!(
            "{}{}{}{}",
            "http://127.0.0.1:", *p, "/registers/", data.addr
        );
        println!("{}", uri);
        let resp = client.get(uri).send().await;
        //we do not care if it returns an error since it will be fail detected
        match resp {
            Ok(response) => {
                println!("reached here!");
                let tryjson = response.json::<Val>().await;
                if let Ok(json) = tryjson {
                    if json.tag > current_tag {
                        println!("json tag is {}", json.tag);
                        current_tag = json.tag;
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    // increment the current tag so we can send write requests to all the nodes
    current_tag += 1;
    //store in our db
    let new_entry = Entry {
        addr: data.addr,
        value: Val {
            tag: current_tag,
            value: data.value.clone(),
        },
    };
    entries.insert(
        data.addr,
        Val {
            tag: current_tag,
            value: data.value.clone(),
        },
    );

    for p in peers {
        let uri = format!("{}{}{}", "http://127.0.0.1:", *p, "/registers/");
        println!("{}", uri);
        let resp = client.post(uri).json(&new_entry).send().await;
        //we do not care if it returns an error since it will be fail detected
        match resp {
            Ok(response) => {
                println!("reached here! {:?}", response);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    Ok(warp::reply::json(&new_entry))
}
