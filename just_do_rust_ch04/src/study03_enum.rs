pub fn run() {
    let gender_list = vec![
        get_customer(10), get_customer(7), get_customer(2)];
    
    for gender in &gender_list {
        match gender {
            Gender::Male{name: n, is_military: b} => {
                println!("Male");
                println!("(name: {}, is_military: {})", n, b);
            },
            Gender::Female{name: n} => {
                println!("Female");
                println!("(name: {})", n);
            }
        }    
    }
}

enum Gender {
    Male { name: String, is_military: bool },
    Female { name: String }
}

fn get_customer(id: i32) -> Gender {
    if id % 2 == 0 {
        Gender::Male { name: "Jeff".to_owned(), is_military: true }
    } else {
        Gender::Female { name: "Alice".to_owned() }
    }
}

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