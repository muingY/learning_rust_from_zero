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

use std::{fs::{self, File}, io::Write};

pub fn run() {
    match divmod(10, 3) {
        Ok((q, r)) => println!("(quotient, remainder) = {:?}", (q, r)),
        Err(e) => println!("Error: {}", e),
    }

    println!("-----------------------------------");

    match write_file("foo.txt") {
        Ok(_) => println!("File writing success"),
        Err(e) => println!("Error: {}", e),
    }

    // write_file("foo2.txt").expect("File writing error");
    // write_file("foo3.txt").unwrap();
    // 두 케이스 다 에러시 패닉은 동일. 에러 메시지만 차이.
    // 절대 실패할 수 없다면 unwrap(). 그 외 실패시 패닉되어야 하면 디버깅 편의를 위해 expect.

    // 패닉 없이 처리하려면?
    // - unwrap_or(...)
    //   : 실패 시 지정한 기본값을 사용하여 넘어감. 패닉 X
    // - unwrap_or_else(...클로저...)
    //   : 실패 시 지정한 작업 수행. 패닉 X
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    println!("port 환경변수가 없다면 8080 사용 : {}", port);

    let config = fs::read_to_string("config.toml")
    .unwrap_or_else(|err| {
        println!("파일 없음: {}", err);
        String::new()
    });
    println!("config.toml : {}", config);
}

/// return (n/d, n%d)
fn divmod(n: i32, d: i32) -> Result<(i32, i32), String> {
    if d == 0 {
        Err("can't divide by zero".to_owned())
    } else {
        Ok((n / d, n % d))
    }
}

fn write_file(f_name: &str) -> Result<(), std::io::Error> {
    let mut f = File::create(f_name)?; // ? : 에러 전파. 자주 사용.
    let _ = f.write_all(b"hello");
    return Ok(());
}