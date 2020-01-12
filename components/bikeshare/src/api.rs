use super::types::{Api, Information, Status};
use attohttpc::{Error, Response};

/// Url of the remote api for non-tests
const BIKESHARE_SERVER: &str = "https://gbfs.urbansharing.com/oslobysykkel.no";

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
    status_with_root(BIKESHARE_SERVER)
}

/// Sends a get request to `https://gbfs.urbansharing.com/oslobysykkel.no/station_information.json`,
/// parses the response as json and deserializes the json response into a bikeshare
/// struct using serde.
pub fn information() -> Result<Api<Information>, Error> {
    information_with_root(BIKESHARE_SERVER)
}

/// Sends a get request to `{root}/station_status.json`,
/// parses the response as json and deserializes the json response into a bikeshare
/// struct using serde.
pub fn status_with_root(root: &str) -> Result<Api<Status>, Error> {
    get(&format!("{}{}", root, "/station_status.json"))?.json()
}

/// Sends a get request to `{root}/station_information.json`,
/// parses the response as json and deserializes the json response into a bikeshare
/// struct using serde.
pub fn information_with_root(root: &str) -> Result<Api<Information>, Error> {
    get(&format!("{}{}", root, "/station_information.json"))?.json()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    use chrono::{DateTime, NaiveDateTime, Utc};
    use mockito::{mock, Matcher};

    lazy_static::lazy_static! {
        static ref DATE_ZERO: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);
        static ref MOCKITO_URL: &'static str = Box::leak(mockito::server_url().into_boxed_str());
    }

    #[test]
    fn status_ep_handles_5xx_reponse() {
        let ep = mock("GET", "/station_status.json")
            .with_status(500)
            .create();

        assert!(status_with_root(&*MOCKITO_URL).is_err());

        ep.assert();
    }

    #[test]
    fn status_ep_handles_4xx_reponse() {
        let ep = mock("GET", "/station_status.json")
            .with_status(404)
            .create();

        assert!(status_with_root(&*MOCKITO_URL).is_err());

        ep.assert();
    }

    #[test]
    fn info_ep_handles_5xx_reponse() {
        let ep = mock("GET", "/station_information.json")
            .with_status(500)
            .create();

        assert!(information_with_root(&*MOCKITO_URL).is_err());

        ep.assert();
    }

    #[test]
    fn info_ep_handles_4xx_reponse() {
        let ep = mock("GET", "/station_information.json")
            .with_status(404)
            .create();

        assert!(information_with_root(&*MOCKITO_URL).is_err());

        ep.assert();
    }

    #[test]
    fn status_ep_requested_with_header() {
        let ep = mock("GET", "/station_status.json")
            .match_header("Client-Identifier", Matcher::Regex(r"^.+-.+$".to_string()))
            .create();

        status_with_root(&*MOCKITO_URL).ok();

        ep.assert();
    }

    #[test]
    fn info_ep_requested_with_header() {
        let ep = mock("GET", "/station_information.json")
            .match_header("Client-Identifier", Matcher::Regex(r"^.+-.+$".to_string()))
            .create();

        information_with_root(&*MOCKITO_URL).ok();

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

        assert_eq!(data, status_with_root(&*MOCKITO_URL).unwrap());

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

        assert_eq!(data, information_with_root(&*MOCKITO_URL).unwrap());

        ep.assert();
    }
}
