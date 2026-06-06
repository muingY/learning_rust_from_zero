pub fn run() {
    let str1 = String::from("Hi");
    let str2 = str1; // move!
    // println!("str 1 : {}", str1); // <- String은 소유권이 move되므로 더이상 str1을 사용할 수 없음
    println!("str 2 : {}", str2);

    let num1 = 42;
    let num2 = num1; // copy!
    println!("num1 : {}", num1); // <- 원시 타입들은 소유권 이동이 아닌 copy로 동작. (= 소유권의 개념이 구지 필요하지 않은 타입 등으로 해석)
    println!("num2 : {}", num2);

    let struct1 = StructA { a: 1 };
    let struct2 = struct1;
    // println!("struct1 : {:?}", struct1.a); // <- StructA는 Copy 트레잇이 구현되어있지 않으므로 copy 불가하며, move 됨.
    println!("struct2(move) : {:?}", struct2.a);

    let struct1 = StructB { a: 1 };
    let struct2 = struct1;
    println!("struct1       : {:?}", struct1.a); // <- StructB는 Copy 트레잇이 구현되어있으므로, copy 가능
    println!("struct2(copy) : {:?}", struct2.a);

    let str1 = String::from("Hi");
    let str2 = str1.clone();
    println!("str1              : {}", str1);
    println!("str2(cloned str1) : {}", str2);

    let struct1 = StructC { a: 11 };
    let struct2 = struct1.clone();
    println!("struct1                 : {}", struct1.a);
    println!("struct2(cloned struct1) : {}", struct2.a);
}

#[derive(Debug)]
struct StructA {
    a: i32
}

#[derive(Debug, Copy, Clone)]
struct StructB {
    a: i32
}

#[derive(Debug)]
struct StructC {
    a: i32
}
// Clone 트레잇 직접 구현 예시
impl Clone for StructC {
    fn clone(&self) -> StructC {
        StructC { a: self.a }
    }
}