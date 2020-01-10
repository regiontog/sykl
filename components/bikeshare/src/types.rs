use std::borrow::Cow;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Both endpoints used by this application has the same top level layout,
/// this struct generalized over them both.
#[derive(Deserialize, Serialize, Debug)]
pub struct Api<T> {
    pub data: ApiData<T>,

    #[serde(with = "ts_seconds")]
    pub last_updated: DateTime<Utc>,
}

/// Both endpoints used by this application has the same data layout with
/// the stations key, this struct generalized over them both.
#[derive(Deserialize, Serialize, Debug)]
pub struct ApiData<T> {
    pub stations: Vec<T>,
}

/// The data inside the station_status endpoint is modelled by this struct
#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    pub station_id: String,
    pub is_installed: u64,
    pub is_renting: u64,
    pub num_bikes_available: u64,
    pub num_docks_available: u64,
    pub is_returning: u64,

    #[serde(with = "ts_seconds")]
    pub last_reported: DateTime<Utc>,
}

/// The data inside the station_information endpoint is modelled by this struct
#[derive(Deserialize, Serialize, Debug)]
pub struct Information {
    pub station_id: String,
    pub name: String,
    pub address: String,
    pub lat: f64,
    pub lon: f64,
    pub capacity: u64,
}

/// The status struct contains "dynamic" content about stations and the
/// information endpoint has the "static" content. This struct represents
/// a join of them both. The status content may be missing.
#[derive(Debug)]
pub struct JoinedStation {
    pub station_id: String,

    // Attributes from information
    pub name: String,
    pub address: String,
    pub lat: f64,
    pub lon: f64,
    pub capacity: u64,

    // Attributes from status
    pub status: Option<JoinedStatus>,
}

/// The status content of `JoinedStation`.
#[derive(Debug)]
pub struct JoinedStatus {
    pub is_installed: u64,
    pub is_renting: u64,
    pub num_bikes_available: u64,
    pub num_docks_available: u64,
    pub is_returning: u64,
    pub last_reported: DateTime<Utc>,
}

impl JoinedStation {
    pub(super) fn new(status: Option<&Status>, info: Information) -> Self {
        JoinedStation {
            name: info.name,
            address: info.address,
            lat: info.lat,
            lon: info.lon,
            capacity: info.capacity,

            status: status.map(|status| JoinedStatus {
                is_installed: status.is_installed,
                is_renting: status.is_renting,
                num_bikes_available: status.num_bikes_available,
                num_docks_available: status.num_docks_available,
                is_returning: status.is_returning,
                last_reported: status.last_reported,
            }),

            station_id: info.station_id,
        }
    }

    /// Gets the number of available docks as a copy-on-write string if status
    /// exists, otherwise return `or` parameter.
    pub fn docks_available_str<'a>(&self, or: Cow<'a, str>) -> Cow<'a, str> {
        self.status
            .as_ref()
            .map(|status| Cow::Owned(status.num_docks_available.to_string()))
            .unwrap_or(or)
    }

    /// Gets the number of available bikes as a copy-on-write string if status
    /// exists, otherwise return `or` parameter.
    pub fn bikes_available_str<'a>(&self, or: Cow<'a, str>) -> Cow<'a, str> {
        self.status
            .as_ref()
            .map(|status| Cow::Owned(status.num_bikes_available.to_string()))
            .unwrap_or(or)
    }
}
