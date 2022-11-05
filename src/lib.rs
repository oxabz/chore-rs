mod generation;
mod serve;

use generation::{generate_callendar};
use warp::{Filter, Reply};

const HOST: &str = "https://chores-rs.shuttleapp.rs";

#[shuttle_service::main]
async fn warp() -> shuttle_service::ShuttleWarp<(impl Reply,)> {
    let route = generate_callendar().or(serve::serve());
    Ok(route.boxed())
}