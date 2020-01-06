mod bikeshare;
fn main() -> Result<(), attohttpc::Error> {
    let response =
        attohttpc::get("https://gbfs.urbansharing.com/oslobysykkel.no/station_status.json")
            .header("Client-Identifier", "erlend.tobiassen-sykl")
            .send()?;

    if response.is_success() {
        let json: bikeshare::StationStatus = response.json()?;
        println!("{:#?}", json);
    };

    Ok(())
}
