    let mut token_map = HashMap::new();
    for elem in &_tokvec {
        let one_tok: Vec<&str> = elem.split(",").collect();
        token_map.insert(one_tok[0], one_tok[1]);
        println!("{:?}", token_map);
    }