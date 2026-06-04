fn main() {
    println!("Hello, world!");

    println!("1 + 2 + 3 + ... + 100 = {}", get_sum(100));
}

fn get_sum(n: u32) -> u32 {
    let mut sum: u32 = 0;

    for i in 1..=n {
        sum += i;
    }   

    return sum; // same as 'sum'
    // sum // same as 'return sum;'
}