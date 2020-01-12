#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use std::collections::HashMap;
use std::path::Path;

use bikeshare::types::{ApiData, Information, JoinedStation, Status};
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

#[datatest::files("tests/cases", {
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
