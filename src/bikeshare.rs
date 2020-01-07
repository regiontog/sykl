use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct Api<T> {
    data: ApiData<T>,

    #[serde(with = "ts_seconds")]
    last_updated: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct ApiData<T> {
    stations: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct Status {
    station_id: String,
    is_installed: u64,
    is_renting: u64,
    num_bikes_available: u64,
    num_docks_available: u64,
    is_returning: u64,

    #[serde(with = "ts_seconds")]
    last_reported: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct Information {
    station_id: String,
    name: String,
    address: String,
    lat: f64,
    lon: f64,
    capacity: u64,
}
