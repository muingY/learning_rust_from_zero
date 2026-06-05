use std::env;

mod study01_ownership;
mod study02_struct;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!(r"실행하고자 하는 예제 넘버를 실행 인자로 입력하세요.
 1 - 소유권
 2 - 구조체");
        return;
    }

    let execute_num: i32 = match args[1].trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error - Arg is not number.");
            return;
        }
    };

    match execute_num {
        1 => study01_ownership::run(),
        2 => study02_struct::run(),
        _ => println!("Error - Wrong number")
    }
}