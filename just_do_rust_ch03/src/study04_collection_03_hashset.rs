use std::collections::HashSet;

pub fn run() {
    //SECTION - 해시셋
    let mut set = HashSet::new();

    set.insert(1); set.insert(2); set.insert(3);
    set.insert(3); set.insert(4); set.insert(5);
    println!("HashSet: {:?}", set);

    let mut set = HashSet::from([1, 2, 3, 3, 4, 5]);
    println!("HashSet from array: {:?}", set);

    set.extend([3, 4, 5, 6]);
    println!("Extended HashSet: {:?}", set);

    for x in set.iter() {
        println!("x: {}", x);
    }

    let a = HashSet::from([1, 2, 3, 4]);
    let b = HashSet::from([3, 4, 5, 6]);
    println!("Set A: {:?}, Set B: {:?}", a, b);
    println!("A union B: {:?}", a.union(&b).collect::<Vec<_>>());
    println!("A intersection B: {:?}", a.intersection(&b).collect::<Vec<_>>());
    println!("A difference B: {:?}", a.difference(&b).collect::<Vec<_>>());
    println!("A symmetric difference B: {:?}", a.symmetric_difference(&b).collect::<Vec<_>>());

    println!("----------------------------------");
}