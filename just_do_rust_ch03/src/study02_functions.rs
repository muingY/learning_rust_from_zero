use std::{ops::{Add, Mul, Sub}};

pub fn run() {
    //NOTE - 클로저
    let add_one = |x: i32| x + 1;
    println!("add_one(2):{}", add_one(2));

    let add_one = |x| x + 1;
    println!("add_one(3):{}", add_one(3));

    let print_hello = || println!("hello");
    print_hello();

    let divmod = |x: i32, y: i32| {
        let q = x / y;
        let r = x % y;
        return (q, r);
    };
    println!("divmod(10, 3): {:?}", divmod(10, 3));

    println!("----------------------------------");

    //NOTE - 구조체 및 메서드
    let vec2: Vec2 = Vec2::new(3.0, 4.0);
    println!("Vec2 (3.0, 4.0): {:?}", vec2);
    println!("Length of Vec2: {}", vec2.length());

    let vec2_a = Vec2::new(3.0, 4.0);
    let vec2_b = Vec2::new(1.0, 2.0);
    println!("Vec2 + Vec2: {:?}", vec2_a + vec2_b);
    println!("Vec2 - Vec2: {:?}", vec2_a - vec2_b);
    println!("Vec2 * Vec2: {:?}", vec2_a * vec2_b);
    println!("Vec2 * f64: {:?}", vec2_a * 2.0);
    println!("f64 * Vec2: {:?}", 2.0 * vec2_a);
    println!("Original Vec2 values: {:?}, {:?}", vec2_a, vec2_b);

    println!("----------------------------------");
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }
}

impl Vec2 {
    // 연관함수
    fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    // 메서드
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

//SECTION - 벡터 연산을 위한 Sub, Mul 트레이트 구현
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, vec2: Vec2) -> Vec2 {
        Vec2 {
            x: self * vec2.x,
            y: self * vec2.y,
        }
    }
}
