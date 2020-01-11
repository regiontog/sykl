#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use std::path::Path;

use bikeshare::types::JoinedStation;
use serde_json::from_reader;
use sykl_formatting::pretty_print_stations;

type TestCase = Vec<JoinedStation>;

#[datatest::files("tests/cases", {
    input in r"^(.*)\.input\.json",
    output = r"${1}.output.txt",
})]
#[test]
fn files_test_paths(input: &Path, output: &str) {
    let expected_output = output;
    let mut output = Vec::new();

    let input: TestCase = from_reader(std::fs::File::open(input).unwrap()).unwrap();

    pretty_print_stations(&mut output, input.iter());
    assert_eq!(std::str::from_utf8(&output).unwrap(), expected_output);
    // assert_eq!(input.replace("input", "output"), output);
}
