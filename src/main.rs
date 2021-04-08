mod file_reader;
use std::collections::HashMap;

fn build_tokens(token_vector: Vec<&str>) -> HashMap<&str, &str> {
    let mut token_map = HashMap::new();
    for elem in &token_vector {
        let one_tok: Vec<&str> = elem.split(",").collect();
        token_map.insert(one_tok[0], one_tok[1]);
    }
    return token_map;
}

fn main() {
    let code = String::from(file_reader::read_source_code("src/source_code.c"));
    let split_code: Vec<&str> = code.split(" ").collect();
    let tokens_str = String::from(file_reader::read_tokens("src/TOKENS.txt"));
    let token_map: HashMap<&str, &str> = build_tokens(tokens_str.split(" ").collect());
    println!("\nRESERVED WORDS: \n{:?}", token_map);
    println!("\nCODE SPLIT: \n{:?}", split_code);
}
