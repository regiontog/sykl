use std::fmt::Write;

use attohttpc::ErrorKind;

mod api;
mod bikeshare;
mod formatting;

fn main() -> std::fmt::Result {
    std::process::exit(handle_error(|| {
        let status = api::status()?;
        let info = api::information()?;

        let stations = bikeshare::join(status.data, info.data);

        formatting::pretty_print_stations(stations.values());

        Ok(())
    })?)
}

/// A "best effort" to print a usable error to the user in case something goes wrong.
fn handle_error<T>(
    f: impl FnOnce() -> Result<T, attohttpc::Error>,
) -> Result<i32, std::fmt::Error> {
    match f() {
        Err(error) => {
            let mut cause = String::new();

            match error.kind() {
                ErrorKind::Json(e) => write!(
                    &mut cause,
                    "Could not deserialize json from endpoint into model:\n{}",
                    e
                )?,
                ErrorKind::Http(e) => write!(&mut cause, "Http protocol error:\n{}", e)?,
                ErrorKind::Io(e) => write!(
                    &mut cause,
                    "Problem with the connection to remote api:\n{}",
                    e
                )?,
                e => write!(&mut cause, "Unknown error: {:?}", e)?,
            }

            println!("Failed to get bikeshare data, if you belive this is a problem with this\napplication please report the error at https://github.com/regiontog/sykl/issues");

            if !cause.is_empty() {
                println!("\nCaused by: {}", cause);
            }

            Ok(-1)
        }
        Ok(_) => Ok(0),
    }
}
