mod api;
mod bikeshare;
mod formatting;

fn main() -> Result<(), attohttpc::Error> {
    let status = api::status()?;
    let info = api::information()?;

    let stations = bikeshare::join(status.data, info.data);

    formatting::pretty_print_stations(&stations);

    Ok(())
}
