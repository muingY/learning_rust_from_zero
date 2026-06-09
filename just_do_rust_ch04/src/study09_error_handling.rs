use std::{fs::File, io::Read, io::Error};

pub fn run() {
    // let _file = match File::open("there_is_no_file.txt") {
    //     Ok(f) => f,
    //     Err(err) => {
    //         // panic!("File open error: {}", err);
    //         println!("File open error: {}", err);
    //         return;
    //     },
    // };
    // println!("error handling...");

    let num_str = "010-1234-5678";

    let mut v: Vec<u32> = Vec::new();
    for c in num_str.chars() {
        match c.to_digit(10) {
            Some(n) => v.push(n),
            None => (),
        }
    }
    println!("v = {:?}", v);

    println!("-----------------------------------");

    let result = match read_name() {
        Ok(n) => n,
        Err(e) => {
            println!("{:?}", e);
            String::from("unknown")
        },
    };
    println!("name = {}", result);

    println!("-----------------------------------");

    println!("3! = {}", factorial(3));
    println!("12! = {}", factorial(12));
    println!("13! = {}", factorial(13)); // overflow 발생. wrapping 설정 추가. (toml 참고. release 모드는 알아서 wrapping 처리됨. true로 바꿀 경우 panic)
}

fn read_name() -> Result<String, Error> {
    let mut name = String::new();
    File::open("name.txt")?.read_to_string(&mut name)?;
    Ok(name)
}

fn factorial(n: i32) -> i32 {
    let mut ans = 1;
    for i in 2..=n {
        ans *= i;
    }
    return ans;
}