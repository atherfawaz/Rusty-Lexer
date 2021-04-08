use std::env;
use std::fs;

pub(crate) fn read_file(file_name: &str) -> String {

    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    return contents;
}
