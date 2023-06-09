use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod db;
mod handlers;
mod models;
mod routes;
mod workers;

#[tokio::main]
async fn main() {
    // unaesthetic commmand line parsing for ports
    let args: Vec<String> = env::args().collect();
    let port: u16 = match args.len() {
        1 => 3030,
        2 => args[1].parse().unwrap_or(3030),
        _ => 3030,
    };

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    //init db and server combo
    let state = db::blank();

    // setup peer tracking
    let peers = db::setup_peers(port.clone());
    let me = db::setup_me(port);

    // initialize hashing function
    let sharder = db::setup_sharding();
    let routes = routes::address_routes(me, state, peers.clone(), sharder);

    //start fail detector
    tokio::task::spawn(async move {
        workers::main_worker(peers).await;
    });

    //serve
    warp::serve(routes).run(socket).await;
}
