use crate::rest::handlers;
use std::convert::Infallible;
use warp::Filter;

pub fn init_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Wrapping filter for logging response time
    // https://docs.rs/warp/0.3.0/warp/filters/log/fn.custom.html#example
    let log = warp::log::custom(|info| {
        debug!(
            "[{} {}] [{}] took: {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
        );
    });

    get_domains().or(get_certificate()).with(log)
}

fn get_domains() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("domains")
        .and(warp::get())
        .and_then(handlers::get_domains)
}

fn get_certificate() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("certificate" / String)
        .and(warp::get())
        .and_then(handlers::get_certificate)
}
