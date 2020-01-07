use super::bikeshare::{Api, Information, Status};
use attohttpc::{Error, Response};

const CLIENT_MODIFIER: &str = "erlend.tobiassen-sykl";

fn get(url: &str) -> Result<Response, Error> {
    attohttpc::get(url)
        .header("Client-Identifier", CLIENT_MODIFIER)
        .send()
}

pub(super) fn status() -> Result<Api<Status>, Error> {
    get("https://gbfs.urbansharing.com/oslobysykkel.no/station_status.json")?.json()
}

pub(super) fn information() -> Result<Api<Information>, Error> {
    get("https://gbfs.urbansharing.com/oslobysykkel.no/station_information.json")?.json()
}
