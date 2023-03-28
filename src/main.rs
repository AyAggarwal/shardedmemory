// static NODES: [&str;3] = ["3030","3031","3032"];
// static NUM_NODES: u8 = 3;

mod db;
mod handlers;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    let state = db::blank();
    let routes = routes::register_routes(state);
    // let write = warp::path("write").and(warp::path::param()).and(warp::path::param()).map(|x,_| x);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
