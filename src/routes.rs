use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::Entry;
use crate::models::Val;

// util
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

// util
fn json_body() -> impl Filter<Extract = (Val,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

// aggregator for all register routes
pub fn register_routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    register_list(db.clone()).or(register_write(db))
}

// GET/registers/
fn register_list(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("registers").and(warp::get()).and(with_db(db)).and_then(handlers::list_registers)
}
// POST/registers with Entry {tag, value} as body
fn register_write(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("registers").and(warp::post()).and(json_body()).and(with_db(db)).and_then(handlers::write_register)
}