// static NODES: [&str;3] = ["3030","3031","3032"];
// static NUM_NODES: u8 = 3;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod db;
mod handlers;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let port: u16 = match args.len() {
        1 => 3030,
        2 => args[1].parse().unwrap_or(3030),
        _ => 3030
    };
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    let state = db::blank();
    let routes = routes::register_routes(state);
    // let write = warp::path("write").and(warp::path::param()).and(warp::path::param()).map(|x,_| x);

    warp::serve(routes)
        .run(socket)
        .await;
}
