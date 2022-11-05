use warp::{filters::BoxedFilter, Filter, Reply};

pub fn serve() -> BoxedFilter<(impl Reply,)>{
    let index_html = warp::get()
        .map(|| warp::reply::html(include_str!("../public/index.html")));
    
    let index_js = warp::path!("index.js")
        .and(warp::fs::file("public/index.js"));
    
    let index_css = warp::path!("index.css")
        .and(warp::fs::file("public/index.css"));

    warp::get().and(index_js.or(index_css).or(index_html)).boxed()
}