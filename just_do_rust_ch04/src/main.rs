use std::env;

mod study01_ownership;
mod study02_struct;
mod study03_enum;
mod study04_option;
mod study05_result;
mod study06_trait;
mod study07_generic;
mod study08_smart_pointer;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!(r"실행하고자 하는 예제 넘버를 실행 인자로 입력하세요.
 1 - 소유권
 2 - 구조체
 3 - 열거형(enum)
 4 - Option
 5 - Result
 6 - trait
 7 - generic
 8 - smart pointer");
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
        3 => study03_enum::run(),
        4 => study04_option::run(),
        5 => study05_result::run(),
        6 => study06_trait::run(),
        7 => study07_generic::run(),
        8 => study08_smart_pointer::run(),
        _ => println!("Error - Wrong number")
    }
}