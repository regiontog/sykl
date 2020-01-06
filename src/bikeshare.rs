use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct StationStatus {
    data: StationData,

    #[serde(with = "ts_seconds")]
    last_updated: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct StationData {
    stations: Vec<Station>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct Station {
    is_installed: u64,
    is_renting: u64,
    num_bikes_available: u64,
    num_bikes_disabled: Option<u64>,
    num_docks_available: u64,
    num_docks_disabled: Option<u64>,
    is_returning: u64,
    station_id: String,

    #[serde(with = "ts_seconds")]
    last_reported: DateTime<Utc>,
}
