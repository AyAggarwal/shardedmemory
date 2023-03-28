use std::convert::Infallible;
use warp::{self, http::StatusCode};

use crate::models::Entry;
use crate::models::Val;
use crate::db::Db;

pub async fn read_register(addr: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    let entries = db.lock().await;
    let value = entries.get(&addr);
    if let Some(x) = value {
        Ok(warp::reply::json(x))
    } else {
        // does not insert to database but reads as nil to user
        // improvement: create rejection flow in routing scheme
        Ok(warp::reply::json(&Val {tag: 0, value: "nil".to_string()}))
    }

 }

 // milestone 1, hardcode addr and perform concurrent reads/writes. 
 pub async fn update_register( body: Entry, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut memory = db.lock().await;
    memory.insert(body.addr,body.value);
    Ok(StatusCode::OK)
 }

 