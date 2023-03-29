use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::db::HashFunction;
use crate::db::Me;
use crate::db::Peers;
use crate::handlers;
use crate::models::Entry;
use crate::models::WriteRequest;

// util to add arguments from main
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_me(me: Me) -> impl Filter<Extract = (Me,), Error = Infallible> + Clone {
    warp::any().map(move || me.clone())
}

fn with_peers(peers: Peers) -> impl Filter<Extract = (Peers,), Error = Infallible> + Clone {
    warp::any().map(move || peers.clone())
}

fn with_shard(
    sharder: HashFunction,
) -> impl Filter<Extract = (HashFunction,), Error = Infallible> + Clone {
    warp::any().map(move || sharder.clone())
}

//util to parse body into handler
fn json_body() -> impl Filter<Extract = (Entry,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn write_body() -> impl Filter<Extract = (WriteRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

// aggregator for all register routes
pub fn address_routes(
    me: Me,
    db: Db,
    peers: Peers,
    sharder: HashFunction,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    address_read(db.clone())
        .or(address_update(db.clone()))
        .or(read(db.clone(), peers.clone()))
        .or(write(me, db, peers, sharder))
        .or(pong())
}

// GET/status
fn pong() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status")
        .and(warp::get())
        .and_then(handlers::pong)
}

// GET/registers/{addr}
fn address_read(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("registers" / u64)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::read_address)
}

// GET/read/{addr}
fn read(
    db: Db,
    peers: Peers,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("read" / u64)
        .and(warp::get())
        .and(with_db(db))
        .and(with_peers(peers))
        .and_then(handlers::read)
}

// POST/registers with Entry {tag, value} as body
fn address_update(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("registers")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_address)
}

// POST/write with Entry {tag, value} as body
fn write(
    me: Me,
    db: Db,
    peers: Peers,
    sharder: HashFunction,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("write")
        .and(warp::post())
        .and(write_body())
        .and(with_me(me))
        .and(with_db(db))
        .and(with_peers(peers))
        .and(with_shard(sharder))
        .and_then(handlers::write)
}
