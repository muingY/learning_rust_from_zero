/*
 * 스마트 포인터
 * : 어떤 데이터에 대한 주솟값과 함께, 이 주소값을 관리하는 메타 데이터도 같이 가지고 있는 데이터 구조.
 * 
 * ex)
 * Box - 데이터가 힙에 저장되도록 강제할 수 있음
 * Rc, Arc - 스택에 있는 다른 두 변수가 같은 힙 공간을 가리킬 수 있음
 *           (= Ref Count)
 * (String, Vec, Box, Rc, Arc, ...)
 */

pub fn run() {
    let b = Box::new(5);
    println!("value = {}", b);

    /*
     * Box를 써서 유용한 경우
     * - 일반적인 방법으로는 컴파일 타임에 데이터 크기를 한정할 수 없어서 에러가 날 때
     * - 특정 트레잇을 구현한 타입을 소유하고자 할 때
     * - 큰 크기의 데이터를 실제 복제하지 않고 소유권을 이동시키고자 할 때
     */

    let x = find_runner(true);
    x.run();

    let mut moving_things: Vec<Box<dyn Moving>> = Vec::new();
    moving_things.push(Box::new(Human));
    moving_things.push(Box::new(Dog));
    moving_things.push(Box::new(Human));
    moving_things.push(Box::new(Dog));
    moving_things.push(Box::new(Human));
    moving_things.push(Box::new(Dog));

    for thing in moving_things {
        thing.run();
    }

    let b = Box::new([1, 2, 3, 4]);
    let b2 = b;
    println!("b <- b2: {:?}", b2);

    let b_clone = b2.clone();
    println!("b_clone: {:?}", b_clone);

    // drop(b2);
    // drop(b_clone);
}

struct Human;
struct Dog;

trait Moving {
    fn run(&self);
}

impl Moving for Human {
    fn run(&self) {
        println!("A human is running.");
    }
}
impl Moving for Dog {
    fn run(&self) {
        println!("A dog is running");
    }
}

fn find_runner(is_human: bool) -> Box<dyn Moving> {
    if is_human {
        Box::new(Human)
    } else {
        Box::new(Dog)
    }
}