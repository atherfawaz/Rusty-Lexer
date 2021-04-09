mod file_reader;
use std::collections::HashMap;

fn build_tokens(token_vector: Vec<&str>) -> HashMap<&str, &str> {
    let mut token_map = HashMap::new();
    for &elem in &token_vector {
        println!("Parsing: {}", elem);
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
    println!("\nCODE SPLIT: \n{:?}\n", split_code);

    let mut tokens = Vec::new();
    println!("BUILDING TOKENS:");
    for &elem in &split_code {
        if !elem.is_empty() {
            match token_map.get(elem) {
                Some(map_found) => tokens.push((elem, map_found)),
                None => println!("Calling manual_parser() for {}", elem),
            }
        }
    }

    println!("\nGENERATED TOKENS: \n{:?}", tokens);
}
