use std::fs;

pub(crate) fn read_source_code(file_name: &str) -> String {
    let mut contents =
        fs::read_to_string(file_name).expect("Something went wrong reading the file");
    contents = contents
        .replace("\n", " ")
        .replace("\t", " ")
        .replace("\r", " ");
    return contents;
}

pub(crate) fn read_tokens(file_name: &str) -> String {
    let mut contents =
        fs::read_to_string(file_name).expect("Something went wrong reading the file");
    contents = contents
        .replace("\r", " ")
        .replace("\t", "")
        .replace("\n", "");
    return contents;
}
