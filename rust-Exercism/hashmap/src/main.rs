use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Bule"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::from("Bule"),String::from("Yellow")];
    // let initial_scores = vec![10,50];
    // let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blus");
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Bule"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // let team_name = String::from("Black");
    // let score = scores.get(&team_name);
    // println!("{:?}", score);

    // scores.entry(String::from("Yellow")).or_insert(12);
    // scores.entry(String::from("Bule")).or_insert(12);
    // for (key, value) in &scores {
    //     println!("{}:{}", key, value);
    // }

    //统计text中词语出现次数并放入hashmap
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map)
}
