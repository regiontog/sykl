use std::sync::RwLock;
use std::time::Duration;

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

fn update_status_sync(stations: web::Data<Stations>) {
    loop {
        std::thread::sleep(Duration::from_secs(10));

        // TODO: Would be better if bikeshare had an async api.
        if let Ok(status) = bikeshare::api::status() {
            match stations.write() {
                Ok(mut stations) => {
                    bikeshare::update_status(&mut stations, status.data);
                }
                e => eprintln!("Poisoned RWLock: {:#?}", e),
            }
        };
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let status = bikeshare::api::status()?;
    let info = bikeshare::api::information()?;

    let stations = RwLock::new(bikeshare::join(status.data, info.data));
    let stations = web::Data::new(stations);
    let stations_update = stations.clone();

    // Run update in own thread because bikeshare api is not
    // async and it might block the async runtime.
    std::thread::spawn(move || {
        update_status_sync(stations_update);
    });

    HttpServer::new(move || {
        App::new()
            .app_data(stations.clone())
            .service(all_stations)
            .service(station)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
