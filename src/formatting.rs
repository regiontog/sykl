use std::borrow::Cow;
use std::iter::Iterator;

use super::bikeshare::JoinedStation;

/// This struct represents a row in the pretty printed output.
#[derive(Clone)]
struct Row<'a> {
    name: Cow<'a, str>,
    docks: Cow<'a, str>,
    bikes: Cow<'a, str>,
}

/// The extra padding to add in addition to the length of the longest value of the column.
const EXTRA_PADDING: usize = 3;

/// The string to use in the case where no status information is available for a station.
const PLACEHOLDER: Cow<'static, str> = Cow::Borrowed("---");

/// The headers used in the pretty print.
const HEADER: Row<'static> = Row {
    name: Cow::Borrowed("NAME"),
    docks: Cow::Borrowed("AVAILABLE DOCKS"),
    bikes: Cow::Borrowed("AVAILABLE BIKES"),
};

/// Prints each stations name, number of available docks and number of available
/// bikes in 3 columns per row with a header row.
pub(super) fn pretty_print_stations<'a>(stations: impl Iterator<Item = &'a JoinedStation> + Clone) {
    // Convert the stations iterator to a iterator of Row structs.
    let rows = stations.map(|station| Row {
        name: Cow::Borrowed(&station.name),
        docks: station.docks_available_str(PLACEHOLDER),
        bikes: station.bikes_available_str(PLACEHOLDER),
    });

    let rows = std::iter::once(HEADER).chain(rows);

    let mut len_name = 0;
    let mut len_docks = 0;
    let mut len_bikes = 0;

    // Iterate once to find the longest string for each column.
    for row in rows.clone() {
        len_name = len_name.max(row.name.len());
        len_docks = len_docks.max(row.docks.len());
        len_bikes = len_bikes.max(row.bikes.len());
    }

    len_name += EXTRA_PADDING;
    len_docks += EXTRA_PADDING;
    len_bikes += EXTRA_PADDING;

    for row in rows {
        println!(
            "{name:len_name$}{docks:len_docks$}{bikes:len_bikes$}",
            name = row.name,
            docks = row.docks,
            bikes = row.bikes,
            len_name = len_name,
            len_docks = len_docks,
            len_bikes = len_bikes,
        )
    }
}
