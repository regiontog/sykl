mod api;
mod bikeshare;

fn main() -> Result<(), attohttpc::Error> {
    let status = api::status()?;
    let info = api::information()?;

    let stations = bikeshare::join(status.data, info.data);

    println!("{:#?}", stations);

    Ok(())
}
