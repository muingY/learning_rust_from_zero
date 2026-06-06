use std::collections::HashMap;

/*
- Option : 어떤 값을 리턴할 때, '값이 없을 수도 있을 때'를 위함.
- Resulrt : 어떤 값을 리턴할 때, '에러가 있을 수도 있을 때'를 위함.

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
   Ok(T),
   Err(E),
}
*/

pub fn run() {
    let map = HashMap::from([
        ("Jeff", 80), ("Alice", 100),
    ]);
    let name_list = vec!["Jeff", "Tom", "Alice", "Bill", "Ana"];

    println!("name searching...");
    for name in name_list {
        match map.get(name) {
            Some(point) => {
                println!("{} found! (point: {})", name, point);
            },
            None => {
                println!("There is no {}.", name);
            },
        }
    }

    println!("-----------------------------------");

    let num_str = "010-1234-5678";

    let mut v: Vec<u32> = Vec::new();
    for c in num_str.chars() {
        match c.to_digit(10) {
            Some(n) => v.push(n),
            None => (),
        }
    }
    println!("v={:?}", v);

    let mut v: Vec<u32> = Vec::new();
    for c in num_str.chars() {
        if let Some(n) = c.to_digit(10) { v.push(n); }
    }
    println!("v={:?}", v);

    let v: Vec<u32> = num_str.chars().filter_map(|c| c.to_digit(10)).collect();
    println!("v={:?}", v);

    // 아직 익숙하지 않아서 그런지, 가장 가독성 좋고 디버깅하기 좋은 코드는 첫번째같다.
}