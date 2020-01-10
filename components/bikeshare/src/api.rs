use super::types::{Api, Information, Status};
use attohttpc::{Error, Response};

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
    get("https://gbfs.urbansharing.com/oslobysykkel.no/station_status.json")?.json()
}

/// Sends a get request to `https://gbfs.urbansharing.com/oslobysykkel.no/station_information.json`,
/// parses the response as json and deserializes the json response into a bikeshare
/// struct using serde.
pub fn information() -> Result<Api<Information>, Error> {
    get("https://gbfs.urbansharing.com/oslobysykkel.no/station_information.json")?.json()
}
