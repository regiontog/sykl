use std::collections::HashMap;

pub mod api;
pub mod types;

use types::*;

/// Joins the data from status and information into a hashmap from `station_id -> JoinedStation`.
/// If a station in the information data is not in the status data the JoinedStation has
/// `Option::None` in its status attribute.
pub fn join(
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
                JoinedStation::new(status.get(&info.station_id), info),
            )
        })
        .collect()
}
