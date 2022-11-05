use warp::{filters::BoxedFilter, Filter, Reply, reply::with_header};

pub fn serve() -> BoxedFilter<(impl Reply,)>{
    let index_html = warp::get()
        .map(|| warp::reply::html(include_str!("../public/index.html")));
    
    let index_js = warp::path!("index.js")
        .map(|| {
            with_header(
                include_str!("../public/index.js"),
                "Content-Type",
                "text/javascript",
            )
        });
    
    let index_css = warp::path!("index.css")
        .map(||{
            with_header(
                include_str!("../public/index.css"),
                "Content-Type",
                "text/css",
            )
        });

    warp::get().and(index_js.or(index_css).or(index_html)).boxed()
}