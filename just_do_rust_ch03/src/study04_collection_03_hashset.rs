use std::collections::HashSet;

pub fn run() {
    //SECTION - 해시셋
    let mut set = HashSet::new();

    set.insert(1); set.insert(2); set.insert(3);
    set.insert(3); set.insert(4); set.insert(5);
    println!("HashSet: {:?}", set);

    println!("----------------------------------");
}