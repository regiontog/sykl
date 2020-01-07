use std::collections::HashMap;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct Api<T> {
    pub(super) data: ApiData<T>,

    #[serde(with = "ts_seconds")]
    pub(super) last_updated: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct ApiData<T> {
    pub(super) stations: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct Status {
    pub(super) station_id: String,
    pub(super) is_installed: u64,
    pub(super) is_renting: u64,
    pub(super) num_bikes_available: u64,
    pub(super) num_docks_available: u64,
    pub(super) is_returning: u64,

    #[serde(with = "ts_seconds")]
    pub(super) last_reported: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct Information {
    pub(super) station_id: String,
    pub(super) name: String,
    pub(super) address: String,
    pub(super) lat: f64,
    pub(super) lon: f64,
    pub(super) capacity: u64,
}

#[derive(Debug)]
pub(super) struct JoinedStation {
    pub(super) station_id: String,

    // Attributes from information
    pub(super) name: String,
    pub(super) address: String,
    pub(super) lat: f64,
    pub(super) lon: f64,
    pub(super) capacity: u64,

    // Attributes from status
    pub(super) status: Option<JoinedStatus>,
}

#[derive(Debug)]
pub(super) struct JoinedStatus {
    pub(super) is_installed: u64,
    pub(super) is_renting: u64,
    pub(super) num_bikes_available: u64,
    pub(super) num_docks_available: u64,
    pub(super) is_returning: u64,
    pub(super) last_reported: DateTime<Utc>,
}

pub(super) fn join(
    status: ApiData<Status>,
    information: ApiData<Information>,
) -> HashMap<String, JoinedStation> {
    let status: HashMap<String, Status> = status
        .stations
        .into_iter()
        .map(|status| (status.station_id.clone(), status))
        .collect();

    information
        .stations
        .into_iter()
        .map(|info| {
            (
                info.station_id.clone(),
                JoinedStation {
                    name: info.name,
                    address: info.address,
                    lat: info.lat,
                    lon: info.lon,
                    capacity: info.capacity,

                    status: status.get(&info.station_id).map(|status| JoinedStatus {
                        is_installed: status.is_installed,
                        is_renting: status.is_renting,
                        num_bikes_available: status.num_bikes_available,
                        num_docks_available: status.num_docks_available,
                        is_returning: status.is_returning,
                        last_reported: status.last_reported,
                    }),

                    station_id: info.station_id,
                },
            )
        })
        .collect()
}
