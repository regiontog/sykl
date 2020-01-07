mod api;
mod bikeshare;

fn main() -> Result<(), attohttpc::Error> {
    println!("{:#?}", api::status()?);
    println!("{:#?}", api::information()?);

    Ok(())
}
