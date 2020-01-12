use std::sync::RwLock;

use actix_http::error::ResponseError;
use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer};

type Stations = RwLock<std::collections::HashMap<String, bikeshare::types::JoinedStation>>;

#[derive(Debug)]
enum InternalError {
    LockError,
}

impl<T> From<std::sync::PoisonError<T>> for InternalError {
    fn from(_error: std::sync::PoisonError<T>) -> Self {
        Self::LockError
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for InternalError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[get("/station/{id}")]
async fn station(
    id: web::Path<String>,
    stations: web::Data<Stations>,
) -> Result<HttpResponse, InternalError> {
    let lock = stations.read()?;

    if let Some(station) = lock.get(&*id) {
        Ok(HttpResponse::Ok().json2(station))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/stations")]
async fn all_stations(stations: web::Data<Stations>) -> Result<HttpResponse, InternalError> {
    Ok(HttpResponse::Ok().json2(&*stations.read()?))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // TODO: Update status periodically so that the i
    // data returned by the endpoint is always up to date.
    let status = bikeshare::api::status()?;
    let info = bikeshare::api::information()?;

    let stations = RwLock::new(bikeshare::join(status.data, info.data));
    let stations = web::Data::new(stations);

    HttpServer::new(move || {
        App::new()
            .app_data(stations.clone())
            .service(all_stations)
            .service(station)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
