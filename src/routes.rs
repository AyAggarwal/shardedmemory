use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::db::Peers;
use crate::handlers;
use crate::models::Entry;
use crate::models::WriteRequest;

// util
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

// util
fn with_peers(peers: Peers) -> impl Filter<Extract = (Peers,), Error = Infallible> + Clone {
    warp::any().map(move || peers.clone())
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
pub fn address_routes(db: Db, peers: Peers) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    address_read(db.clone()).or(address_update(db.clone())).or(read(db.clone(), peers.clone())).or(write(db, peers)).or(pong())
}

// GET/status
fn pong() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status").and(warp::get()).and_then(handlers::pong)
}

// GET/registers/{addr}
fn address_read(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("registers" / u64).and(warp::get()).and(with_db(db)).and_then(handlers::read_address)
}

// GET/read/{addr}
fn read(db: Db, peers: Peers) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("read" / u64).and(warp::get()).and(with_db(db)).and(with_peers(peers)).and_then(handlers::read)
}

// POST/registers with Entry {tag, value} as body
fn address_update(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("registers").and(warp::post()).and(json_body()).and(with_db(db)).and_then(handlers::update_address)
}

// POST/write with Entry {tag, value} as body
fn write(db: Db, peers: Peers) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("write").and(warp::post()).and(write_body()).and(with_db(db)).and(with_peers(peers)).and_then(handlers::write)
}