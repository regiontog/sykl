use super::types::{Api, Information, Status};
use attohttpc::{Error, Response};

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const BIKESHARE_SERVER: &str = "https://gbfs.urbansharing.com/oslobysykkel.no";

#[cfg(test)]
lazy_static::lazy_static! {
    /// Url of the remote api
    static ref BIKESHARE_SERVER: &'static str = Box::leak(mockito::server_url().into_boxed_str());
}

/// The Client-Identifier of this application.
const CLIENT_IDENTIFIER: &str = "erlend.tobiassen-sykl";

/// Sends a get request to a specific url with the Client-Identifier header set to
/// the application id.
fn get(url: &str) -> Result<Response, Error> {
    attohttpc::get(url)
        .header("Client-Identifier", CLIENT_IDENTIFIER)
        .send()
}

/// Sends a get request to `https://gbfs.urbansharing.com/oslobysykkel.no/station_status.json`,
/// parses the response as json and deserializes the json response into a bikeshare
/// struct using serde.
pub fn status() -> Result<Api<Status>, Error> {
    get(&format!("{}{}", &*BIKESHARE_SERVER, "/station_status.json"))?.json()
}

/// Sends a get request to `https://gbfs.urbansharing.com/oslobysykkel.no/station_information.json`,
/// parses the response as json and deserializes the json response into a bikeshare
/// struct using serde.
pub fn information() -> Result<Api<Information>, Error> {
    get(&format!(
        "{}{}",
        &*BIKESHARE_SERVER, "/station_information.json"
    ))?
    .json()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    use chrono::{DateTime, NaiveDateTime, Utc};
    use mockito::{mock, Matcher};

    lazy_static::lazy_static! {
        static ref DATE_ZERO: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);
    }

    #[test]
    fn status_ep_requested_with_header() {
        let ep = mock("GET", "/station_status.json")
            .match_header("Client-Identifier", Matcher::Regex(r"^.+-.+$".to_string()))
            .create();

        status().ok();

        ep.assert();
    }

    #[test]
    fn info_ep_requested_with_header() {
        let ep = mock("GET", "/station_information.json")
            .match_header("Client-Identifier", Matcher::Regex(r"^.+-.+$".to_string()))
            .create();

        information().ok();

        ep.assert();
    }

    #[test]
    fn status_parses_valid_model() {
        let data = Api {
            data: ApiData {
                stations: vec![Status {
                    station_id: "κόσμε".to_string(),
                    is_installed: 1,
                    is_renting: 2,
                    num_bikes_available: 10,
                    num_docks_available: 90,
                    is_returning: 10,
                    last_reported: DATE_ZERO.clone(),
                }],
            },
            last_updated: DATE_ZERO.clone(),
        };

        let body = serde_json::to_string(&data).unwrap();

        eprintln!("{}", body);
        eprintln!("{:#?}", serde_json::from_str::<Api<Status>>(&body).unwrap());

        let ep = mock("GET", "/station_status.json")
            .with_status(200)
            .with_header("content-type", "application/json; charset=UTF-8")
            .with_body(body)
            .create();

        assert_eq!(data, status().unwrap());

        ep.assert();
    }

    #[test]
    fn info_parses_valid_model() {
        let data = Api {
            data: ApiData {
                stations: vec![Information {
                    station_id: "hello".to_string(),
                    name: "hello2".to_string(),
                    address: "Måneveien".to_string(),
                    lat: 1.,
                    lon: 2.,
                    capacity: 100,
                }],
            },
            last_updated: DATE_ZERO.clone(),
        };

        let body = serde_json::to_string(&data).unwrap();

        eprintln!("{}", body);
        eprintln!(
            "{:#?}",
            serde_json::from_str::<Api<Information>>(&body).unwrap()
        );

        let ep = mock("GET", "/station_information.json")
            .with_status(200)
            .with_header("content-type", "application/json; charset=UTF-8")
            .with_body(body)
            .create();

        assert_eq!(data, information().unwrap());

        ep.assert();
    }
}
