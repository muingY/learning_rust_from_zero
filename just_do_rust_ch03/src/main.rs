use std::env;

mod study01_data_type;
mod study02_functions;
mod study03_control_flow;
mod study04_collection_01_vector;
mod study04_collection_02_hashmap;
mod study04_collection_03_hashset;
mod study05_str;
mod study06_string;
mod study07_iterator;
mod study08_iterator_method;

fn main() {
    let args: Vec<String> = env::args().collect(); //NOTE - 프로그램 실행 시 인자 받기
    
    if args.len() <= 1 {
        println!("Usage: 실행하려는 학습 예제 넘버");
        println!(" 1 : 데이터 타입 및 기본 사용법");
        println!(" 2 : 함수, 클로저, 구조체 및 메서드");
        println!(" 3 : 제어문");
        println!(" 4 : 컬렉션 - 벡터");
        println!(" 5 : 컬렉션 - 해시맵");
        println!(" 6 : 컬렉션 - 해시셋");
        println!(" 7 : 문자열 - &str");
        println!(" 8 : 문자열 - string");
        println!(" 9 : 이터레이터");
        println!(" 10 : 이터레이터 - 메서드");
        return;
    }

    let run_example: u8;
    run_example = args[1].parse().expect("ERR : Please provide a valid number");
    
    match run_example {
        1 => study01_data_type::run(),
        2 => study02_functions::run(),
        3 => study03_control_flow::run(),
        4 => study04_collection_01_vector::run(),
        5 => study04_collection_02_hashmap::run(),
        6 => study04_collection_03_hashset::run(),
        7 => study05_str::run(),
        8 => study06_string::run(),
        9 => study07_iterator::run(),
        10 => study08_iterator_method::run(),
        _ => println!("Invalid example number. Please choose 1, 2, or 3."),
    }
}
