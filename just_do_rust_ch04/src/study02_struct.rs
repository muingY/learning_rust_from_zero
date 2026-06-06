pub fn run() {
    let mut s1 = Student {
        name: "Jeff".to_owned(),
        point: 80,
        is_male: false,
    };

    println!("name={}, point={}, is_male={}", s1.name, s1.point, s1.is_male);

    s1.point = 100;
    println!("s1: {:?}", s1);

    let s2 = Student::default();
    println!("s2: {:?}", s2);

    println!("-----------------------------------");

    let p1 = Point(0, 0);
    let p2 = Point(3, 4);
    let dist = cal_distance(&p1, &p2);
    println!("p1({:?}) - p2({:?}) distance = {}", p1, p2, dist);

    println!("-----------------------------------");

    let p1 = Vec2::new(0, 0);
    let p2 = Vec2::new(3, 4);
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
    println!("p1 - p2 distance = {}", p1.distance(&p2));

    println!("-----------------------------------");

    let m = Marker;
    let d = Data(42);

    do_something(&m);
    do_something(&d);
}

// 일반적인 구조체
#[derive(Debug, Default)]
struct Student {
    name: String,
    point: i32,
    is_male: bool,
}

// 튜플 구조체
// struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32);

fn cal_distance(p1: &Point, p2: &Point) -> f64 {
    (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f64).sqrt()
}

// 구조체 + 메서드
#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    fn distance(&self, p: &Vec2) -> f64 {
        (((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f64).sqrt()
    }
}

// 빈 구조체
// - 타입을 구분하고자 할 떄
// - 필드값 없이 메서드만 구현하고 싶을 때
struct Marker;
struct Data(i32);

trait MarkerTrait {
    fn whoami(&self);
}

impl MarkerTrait for Marker {
    fn whoami(&self) {
        println!("I am a Marker (unit-like struct, no data)");
    }
}
impl MarkerTrait for Data {
    fn whoami(&self) {
        println!("I am Data, value = {}", self.0);
    }
}

fn do_something<T: MarkerTrait>(item: &T) {
    item.whoami();
}