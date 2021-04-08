mod file_reader;

fn main() {
    let source_code = String::from(file_reader::read_file("src/source_code.c"));
    println!("Source code: \n{}", source_code);
}
