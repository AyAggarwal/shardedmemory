use std::convert::Infallible;
use warp::{self, http::StatusCode};

use crate::models::Entry;
use crate::models::Val;
use crate::db::Db;

pub async fn list_registers(db: Db) -> Result<impl warp::Reply, Infallible> {
    let entries = db.lock().await;
    let entries: Vec<Val> = entries.clone().into_values().collect();
    Ok(warp::reply::json(&entries))
 }

 pub async fn write_register( body: Val, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut memory = db.lock().await;
    memory.insert(1,body);
    Ok(StatusCode::CREATED)
 }