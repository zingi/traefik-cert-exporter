use crate::acme_model::{Acme, AcmeError, Certificate};
use crate::export;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn get_domains() -> Result<Box<dyn warp::Reply>, Infallible> {
    // read acme.json
    let acme: Result<Acme, AcmeError> = export::read_acme_json().await;

    // check if reading was successful
    if acme.is_err() {
        return Ok(Box::new(warp::reply::with_status(
            format!("{}", acme.err().unwrap()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )));
    }
    let acme: Acme = acme.unwrap();

    // read domains
    let domains = acme.get_http().get_domains();

    Ok(Box::new(warp::reply::json(&domains)))
}

pub async fn get_certificate(name: String) -> Result<Box<dyn warp::Reply>, Infallible> {
    // read acme.json
    let acme: Result<Acme, AcmeError> = export::read_acme_json().await;

    // check if reading was successful
    if acme.is_err() {
        return Ok(Box::new(warp::reply::with_status(
            format!("{}", acme.err().unwrap()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )));
    }
    let acme: Acme = acme.unwrap();

    let cert = acme.get_http().get_certificate(&name);

    return match cert {
        None => return Ok(Box::new(StatusCode::NOT_FOUND)),
        Some(c) => {
            return Ok(Box::new(warp::reply::with_status(
                format!("{}", c.get_certificate().unwrap()),
                StatusCode::OK,
            )))
        }
    };
}
