mod generation;
mod serve;

use warp::Filter;
use generation::{generate_callendar};
use serve::{serve};

const HOST: &str = "localhost:3030";

#[tokio::main]
async fn main() {
    let routes = generate_callendar().or(serve());
    let routes = routes.with(warp::log("chores"));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}