pub fn run() {
    //SECTION - 벡터
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("Vector v: {:?}", v);

    let v = vec![5, 6, 7];
    println!("Vector v: {:?}", v);

    let v = vec![3; 5];
    println!("Vector v: {:?}", v);

    let arr = [3, 4];
    let arr_2_v = arr.to_vec();
    println!("Array arr: {:?}, Array to Vector: {:?}", arr, arr_2_v);

    let mut v = vec![1, 2, 3];
    println!("Original Vector v: {:?}", v);
    v.extend([4, 5, 6, 7, 8]);
    println!("Extended Vector v: {:?}", v);

    println!("vec reading : ");
    for a in &v {
        print!("{}, ", a);
    }
    println!();

    v.pop();
    println!("After pop: {:?}", v);
    v.push(111);
    println!("After push(111): {:?}", v);

    v.append(&mut vec![222, 333]);
    println!("After append: {:?}", v);
    v.clear();
    println!("After clear: {:?}", v);
    println!("vec length: {}, capacity: {}", v.len(), v.capacity());

    match v.is_empty() {
        true => println!("Vector is empty"),
        false => println!("Vector is not empty"),
    }

    v.append(&mut vec![1, 2, 4]);
    println!("After append: {:?}", v);
    v.shrink_to_fit();
    println!("After shrink_to_fit: {:?}", v);
    println!("vec length: {}, capacity: {}", v.len(), v.capacity());

    v.insert(0, 0);
    println!("After insert(0, 0): {:?}", v);
    v.insert(3, 3);
    println!("After insert(3, 3): {:?}", v);

    match v.pop() {
        Some(value) => println!("Popped value: {}", value),
        None => println!("Vector is empty"),
    }
    println!("After pop: {:?}", v);

    v.remove(0);
    println!("After remove(0): {:?}", v);

    v.resize(10, 0);
    println!("After resize(10): {:?}", v);
    v.resize(5, 0);
    println!("After resize(5, 0): {:?}", v);

    v.truncate(2);
    println!("After truncate(2): {:?}", v);

    v.fill(999);
    println!("After fill(999): {:?}", v);

    v.clear();
    v.extend([1, 2, 3, 4, 5]);

    v.reverse();
    println!("After reverse: {:?}", v);

    v.sort();
    println!("After sort: {:?}", v);    

    println!("----------------------------------");
}