use std::borrow::Cow;
use std::collections::HashMap;

use super::bikeshare::JoinedStation;

#[derive(Clone)]
struct Row<'a> {
    name: Cow<'a, str>,
    docks: Cow<'a, str>,
    bikes: Cow<'a, str>,
}

const EXTRA_PADDING: usize = 3;
const PLACEHOLDER: Cow<'static, str> = Cow::Borrowed("---");

const HEADER: Row<'static> = Row {
    name: Cow::Borrowed("NAME"),
    docks: Cow::Borrowed("AVAILABLE DOCKS"),
    bikes: Cow::Borrowed("AVAILABLE BIKES"),
};

pub(super) fn pretty_print_stations(stations: &HashMap<String, JoinedStation>) {
    let rows = stations.values().map(|station| Row {
        name: Cow::Borrowed(&station.name),
        docks: station
            .status
            .as_ref()
            .map(|status| Cow::Owned(status.num_docks_available.to_string()))
            .unwrap_or(PLACEHOLDER),
        bikes: station
            .status
            .as_ref()
            .map(|status| Cow::Owned(status.num_bikes_available.to_string()))
            .unwrap_or(PLACEHOLDER),
    });

    let rows = std::iter::once(HEADER).chain(rows);

    let len_name = rows
        .clone()
        .map(|row| row.name.len())
        .max()
        .expect("There should atleast be a header in rows")
        + EXTRA_PADDING;

    let len_docks = rows
        .clone()
        .map(|row| row.docks.len())
        .max()
        .expect("There should atleast be a header in rows")
        + EXTRA_PADDING;

    let len_bikes = rows
        .clone()
        .map(|row| row.bikes.len())
        .max()
        .expect("There should atleast be a header in rows")
        + EXTRA_PADDING;

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
