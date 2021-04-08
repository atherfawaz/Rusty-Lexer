mod file_reader;

fn main() {
    let source_code = String::from(file_reader::read_file("src/source_code.c"));
    println!("SOURCE CODE: \n{}", source_code);

    let mut split = source_code.split(" ");

    let vec: Vec<&str> = split.collect();

    println!("Tokens: \n{}", 1);

}
