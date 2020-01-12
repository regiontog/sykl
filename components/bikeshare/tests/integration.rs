#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use std::collections::HashMap;
use std::path::Path;

use bikeshare::types::{ApiData, Information, JoinedStation, Status};
use mockito::mock;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::from_reader;

#[derive(Deserialize)]
struct TestCase {
    statuses: Vec<Status>,
    information: Vec<Information>,
}

type TestOutput = HashMap<String, JoinedStation>;

fn read<T: DeserializeOwned>(path: &Path) -> T {
    from_reader(std::fs::File::open(path).unwrap()).unwrap()
}

#[datatest::files("tests/cases/join", {
    input in r"^(.*)\.input\.json",
    output = r"${1}.output.json",
})]
#[test]
fn files_test_join(input: &Path, output: &Path) {
    let input: TestCase = read(input);
    let expected_output: TestOutput = read(output);

    let output = bikeshare::join(
        ApiData {
            stations: input.statuses,
        },
        ApiData {
            stations: input.information,
        },
    );

    assert_eq!(output, expected_output);
}

#[datatest::data("tests/cases/bad_info_input.yml")]
#[test]
fn info_bad_input_is_handled(case: String) {
    let ep = mock("GET", "/station_information.json")
        .with_status(200)
        .with_header("content-type", "application/json; charset=UTF-8")
        .with_body(&case)
        .create();

    assert!(
        bikeshare::api::information_with_root(&mockito::server_url())
            .map(|r| eprintln!("{:#?}", r))
            .is_err()
    );

    ep.assert();
}

#[datatest::data("tests/cases/bad_status_input.yml")]
#[test]
fn status_bad_input_is_handled(case: String) {
    let ep = mock("GET", "/station_status.json")
        .with_status(200)
        .with_header("content-type", "application/json; charset=UTF-8")
        .with_body(&case)
        .create();

    assert!(bikeshare::api::status_with_root(&mockito::server_url())
        .map(|r| eprintln!("{:#?}", r))
        .is_err());

    ep.assert();
}
