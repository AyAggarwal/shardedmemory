use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::Entry;
use crate::models::WriteRequest;

// util
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

// util
fn with_port(port: u16) -> impl Filter<Extract = (u16,), Error = Infallible> + Clone {
    warp::any().map(move || port.clone())
}

// util
fn json_body() -> impl Filter<Extract = (Entry,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

// util
fn write_body() -> impl Filter<Extract = (WriteRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

// aggregator for all register routes
pub fn address_routes(db: Db, port: u16) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    address_read(db.clone()).or(address_update(db.clone())).or(read(db.clone(), port)).or(write(db, port))
}

// GET/registers/{addr}
fn address_read(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("registers" / u64).and(warp::get()).and(with_db(db)).and_then(handlers::read_address)
}

// GET/read/{addr}
fn read(db: Db, port: u16) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("read" / u64).and(warp::get()).and(with_db(db)).and(with_port(port)).and_then(handlers::read)
}

// POST/registers with Entry {tag, value} as body
fn address_update(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("registers").and(warp::post()).and(json_body()).and(with_db(db)).and_then(handlers::update_address)
}

// POST/write with Entry {tag, value} as body
fn write(db: Db, port: u16) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("write").and(warp::post()).and(write_body()).and(with_db(db)).and(with_port(port)).and_then(handlers::write)
}