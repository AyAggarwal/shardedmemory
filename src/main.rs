use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod db;
mod handlers;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    // unaesthetic commmand line parsing for ports
    let args: Vec<String> = env::args().collect();
    let port: u16 = match args.len() {
        1 => 3030,
        2 => args[1].parse().unwrap_or(3030),
        _ => 3030
    };

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    //init db and server combo
    let state = db::blank();
    let routes = routes::address_routes(state);

    //serve
    warp::serve(routes)
        .run(socket)
        .await;
}
