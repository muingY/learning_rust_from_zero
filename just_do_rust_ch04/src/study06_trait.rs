pub fn run() {
    let s = ExStruct {};
    let a: <ExStruct as ExTrait>::TypeNoDefault = 1.0;

    println!("a = {}", a);

    println!("CONST_DEFAULT={}", ExStruct::CONST_DEFAULT);
    println!("CONST_NO_DEFAULT={}", ExStruct::CONST_NO_DEFAULT);

    s.method_default();
    s.method_without_default();

    println!("-----------------------------------");

    let truck = Truck {};
    let suv = SUV {};
    let sedan = Sedan {};

    // dyn Car = 정확한 타입은 모르겠고, 최소한 Car trait을 구현한 값이라는 의미
    let cars: Vec<&dyn Car> = vec![&truck, &suv, &sedan]; // 동적 바인딩

    // 정적 바인딩
    fn binding(car: &impl Car) {
        car.derive();
    }
    fn binding2<T: Car>(car: &T) {
        car.derive();
    }
    binding(&truck);
    binding(&suv);
    binding2(&sedan);

    for car in &cars {
        car.derive();
    }

    println!("-----------------------------------");

    let p1 = Point::new(3, 4);
    let p2 = Point::new(10, 10);
    
    println!("p1: {:?}, p2: {:?}", p1, p2);
    println!("p1 + p2 = {:?}", p1 + p2);
    println!("p1 - p2 = {:?}", p1 - p2);
}

/*
 * Rust의 트레잇은 타입, 상수, 함수를 항목으로 가질 수 있음.
 */
trait ExTrait {
    // 공통으로 사용할 수 있는 타입 정의. 구현측에서 타입 지정을 해야함.
    type TypeNoDefault;

    // 상수값 정의.
    const CONST_DEFAULT: i32 = 100; // 구현측에서 재할당 가능.
    const CONST_NO_DEFAULT: i32; // 구현측에서 값을 할당해야 함.

    // 함수 정의.
    fn method_default(&self) {
        println!("default method");
    } // 함수의 본체까지 정의. 구현 측에서 다시 구현할 수 있음.
    fn method_without_default(&self); // 공통 메서드 정의. 구현하는 측에서 함수 바디 구현해야 함.
}

struct ExStruct {}

impl ExTrait for ExStruct {
    type TypeNoDefault = f64; // 타입 지정

    const CONST_DEFAULT: i32 = 101; // 값 재할당
    const CONST_NO_DEFAULT: i32 = 102; // 값 할당

    // 메소드 재구현
    fn method_default(&self) {
        println!("method is re-implemented.");
    }
    // 메소드 구현
    fn method_without_default(&self) {
        println!("method_without_default is implemented.");
    }
}

trait Car {
    fn derive(&self);
}

struct Truck {}
impl Car for Truck {
    fn derive(&self) {
        println!("Truck is driving");
    }
}
struct SUV {}
impl Car for SUV {
    fn derive(&self) {
        println!("SUV is driving");
    }
}
struct Sedan {}
impl Car for Sedan {
    fn derive(&self) {
        println!("Sedan is driving");
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32, y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}