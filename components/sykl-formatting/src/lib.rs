use std::borrow::Cow;
use std::iter::Iterator;

use bikeshare::types::JoinedStation;
use unicode_width::*;

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
pub fn pretty_print_stations<'a>(
    mut out: impl std::io::Write,
    stations: impl Iterator<Item = &'a JoinedStation> + Clone,
) -> std::io::Result<()> {
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
        len_name = len_name.max(row.name.width());
        len_docks = len_docks.max(row.docks.width());
        len_bikes = len_bikes.max(row.bikes.width());
    }

    len_name += EXTRA_PADDING;
    len_docks += EXTRA_PADDING;
    len_bikes += EXTRA_PADDING;

    let mut padding_name = String::new();
    let mut padding_docks = String::new();
    let mut padding_bikes = String::new();

    for row in rows {
        padding_name = pad_into_buffer(padding_name, &row.name, len_name, ' ');
        padding_docks = pad_into_buffer(padding_docks, &row.docks, len_docks, ' ');
        padding_bikes = pad_into_buffer(padding_bikes, &row.bikes, len_bikes, ' ');

        writeln!(
            out,
            "{name}{padding_name}{docks}{padding_docks}{bikes}{padding_bikes}",
            name = row.name,
            docks = row.docks,
            bikes = row.bikes,
            padding_name = padding_name,
            padding_docks = padding_docks,
            padding_bikes = padding_bikes,
        )?;
    }

    Ok(())
}

/// # Panics
/// Panics if pad is not a single width character
pub fn pad(unicode: &str, width: usize, pad: char) -> String {
    pad_into_buffer(String::new(), unicode, width, pad)
}

/// # Panics
/// Panics if pad is not a single width character
pub fn pad_into_buffer(mut buffer: String, unicode: &str, width: usize, pad: char) -> String {
    assert_eq!(pad.width(), Some(1));

    let padding_size = width.checked_sub(unicode.width()).unwrap_or(0);
    buffer.truncate(0);
    buffer.reserve(padding_size * pad.len_utf8());

    for _ in 0..padding_size {
        buffer.push(pad);
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_to_long() {
        assert_eq!(pad("to_long_string", 3, ' '), "");
    }

    #[test]
    fn test_pad() {
        assert_eq!(pad("hello", 10, ' '), "     ");
    }

    #[test]
    fn test_pad_double_width() {
        assert_eq!(pad("🙌", 4, ' '), "  ");
    }
}
