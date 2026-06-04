use std::collections::HashMap;

pub fn run() {
    //SECTION - 해시맵
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("Jeff", 100);
    map.insert("Tom", 90);
    println!("HashMap map: {:?}", map);

    let mut map = HashMap::from([
        ("Alice", 85),
        ("Bob", 92),
        ("Charlie", 78),
    ]);
    println!("HashMap map: {:?}", map);
    println!("Alice's score: {}", map["Alice"]);
    println!("Bob's score: {}", map.get("Bob").unwrap());
    println!("aaaa's score: {}", map.get("aaaa").unwrap_or(&0));

    map.insert("aaaa", 80);
    println!("HashMap map after inserting 'aaaa': {:?}", map);

    for (k, val) in &map {
        println!("{}: {}", k, val);
    }

    for k in map.keys() {
        if k.starts_with("Ch") {
            println!("Key starting with 'Ch': {}: {}", k, map[k]);
        }
    }

    map.insert("Bob", 100);
    println!("HashMap map after updating 'Bob': {:?}", map);

    let new_data = [("aaaa", 90), ("Dave", 88)];
    for (k, v) in new_data {
        map.entry(k).or_insert(v);
    }
    println!("HashMap map after entry API: {:?}", map);

    println!("HashMap contains 'Alice': {}", map.contains_key("Alice"));
    println!("HashMap contains 'Eve': {}", map.contains_key("Eve"));

    map.remove("aaaa");
    println!("HashMap map after removing 'aaaa': {:?}", map);

    println!("----------------------------------");
}