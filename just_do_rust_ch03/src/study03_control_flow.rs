pub fn run() {
    //SECTION - if / else if / else
    let n = 5;
    println!("n: {}", n);
    if n > 5 {
        println!("n is larger than 5");
    } else if n == 5 {
        println!("n is equal to 5");
    } else {
        println!("n is not larger than 5");
    }

    println!("----------------------------------");

    //SECTION - match
    let age = 32;
    println!("age: {}", age);
    let age_group =match age {
        0_i32..=10_i32 => "baby",
        11_i32..=20_i32 => "teenager",
        21_i32..=60_i32 => "adult",
        _ => "senior",
    };
    println!("age group: {}", age_group);

    let c= 'a';
    println!("c: {}", c);
    let num = match c.to_digit(10) {
        Some(digit) => digit,
        None => {
            println!("Not a valid digit character");
            0
        }
    };
    println!("c -> num: {}", num);

    println!("----------------------------------");

    //SECTION - if let
    let c = '6';
    if let Some(num) = c.to_digit(10) {
        println!("num = {}", num);
    } else {
        println!("Not a valid digit character");
    }

    println!("----------------------------------");

    //SECTION - for
    println!("1..=5 for result:");
    for i in 1..=5 {
        print!("{}, ", i);
    }
    println!();
    for i in 1..=100 {
        if i > 90 {
            println!("Breaking out of the loop at i = {}", i);
            break;
        }
    }
    'label_i : for i in 1..=100 {
        for j in 1..=100 {
            if i * j > 100 {
                println!("Breaking out of the inner loop at i = {}, j = {}", i, j);
                break 'label_i;
            }
        }
    }
    let v = vec![1, 2, 3, 4, 5];
    println!("Vector v: {:?}", v);
    for val in v.iter() { // = for val in &v
        println!("val: {}", val);
    }

    println!("----------------------------------");

    //SECTION - loop
    let mut overheat = 70;
    loop {
        println!("overheat: {}%", overheat);
        if overheat > 90 {
            println!("Overheated! Breaking out of the loop.");
            break;
        }
        overheat += 5;
    }

    println!("----------------------------------");

    //SECTION - while
    let mut overheat = 70;
    while overheat <= 90 {
        println!("overheat: {}%", overheat);
        overheat += 5;
    }
    println!("Overheated! Exiting the loop.");
}