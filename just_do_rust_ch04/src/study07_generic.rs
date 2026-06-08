pub fn run() {
    let p1 = Point { x: 2, y: 3 };
    let mut p2 = Point { x: 2.5, y: 3.5 };
    p2.x = 2.7;
    p2.y = 3.7;

    println!("p1 = {:?}", p1);
    println!("p2 = {:?}", p2);

    println!("-----------------------------------");

    println!("min(3, 1) = {}", min(3, 1));
    println!("min(3.14, 7.12) = {}", min(3.14, 7.12));

    println!("add_sub(3, 5) = {:?}", add_sub(3, 5));
    println!("add_sub(1.2, 0.2) = {:?}", add_sub(1.2, 0.2));

    println!("-----------------------------------");

    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 2.5, y: 3.5};
    let p3 = Point { x: 3, y: 4};

    println!("p1 = ({},{})", p1.x, p1.y);
    println!("p2 = ({},{})", p2.x, p2.y);
    println!("p3 = ({},{})", p3.x, p3.y);

    let p4 = p2.add(&p2);
    println!("p4 = ({},{})", p4.x, p4.y);
    let p4 = p1.add(&p3);
    println!("p4 = ({},{})", p4.x, p4.y);

    println!("-----------------------------------");

    let a = 26;
    let b = 5;
    match divide(a, b) {
        Some(result) => println!("result = {}", result),
        None => println!("can not divide by 0"),
    }

    // use std::fs::File;

    // let f = File::open("asdf.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("There was a problem opening the file: {:?}", err)
    //     }
    // };

    println!("-----------------------------------");

    // 라이프타임 문제를 위해 의도적으로 String 선언 후, as_str()으로 사용
    let x = String::from("123");
    let y = String::from("45678");
    let s = longest(x.as_str(), y.as_str());
    println!("longest is : {}", s);
}

#[derive(Debug)]
struct Point<T> {
    x: T, y: T
}
// impl<T> Point<T> {
//     fn add(&self, rhs: &Point<T>) -> Point<T> 
//         where T: std::ops::Add<Output = T> + Copy
//     {
//         let x_val = self.x + rhs.x;
//         let y_val = self.y + rhs.y;
//         return Point { x: x_val, y: y_val };
//     }
// }
impl Point<i32> {
    fn add(&self, rhs: &Point<i32>) -> Point<i32> {
        let x_val = self.x + rhs.x;
        let y_val = self.y + rhs.y;
        return Point { x: x_val, y: y_val };
    }
}
impl Point<f64> {
    fn add(&self, rhs: &Point<f64>) -> Point<f64> {
        let x_val = self.x + rhs.x;
        let y_val = self.y + rhs.y;
        return Point { x: x_val, y: y_val };
    }
}

fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn add_sub<T>(a: T, b: T) -> (T, T) 
    where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy
{
    return (a + b, a - b);
}

fn divide(n: i32, d: i32) -> Option<i32> {
    if d == 0 {
        None
    } else {
        Some(n / d)
    }
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 }
    else { s2 }
}

/*
솔직히 rust의 라이프타임에 대해서는 이해하기가 조금 어렵다.
어떠한 상황에서 컴파일러가 수명을 판단하지 못해 필요한지는 알겠는데, 그 너머와 실제 사용 예시를 잘 이해하지 못했다.
내가 짜본 대부분의 구조에서는 발생할 일이 없는 것 같아서 더 이해가 힘든것 같다..
*/