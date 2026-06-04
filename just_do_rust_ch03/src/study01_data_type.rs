const CONST_VALUE: u32 = 42;

pub fn run() {
    //NOTE - 변수 및 상수 사용하기
    println!("Constant Value: {}", CONST_VALUE);
    let num1 = 10;
    let mut num2 = 20;
    println!("num1: {}, num2: {}", num1, num2);
    num2 += num1;
    println!("Updated num2: {}", num2);

    println!("----------------------------------");

    //NOTE - 튜플
    let tuple = (1, "This is tuple", 3.14);
    println!("Tuple: {:?}", tuple);
    println!("First element: {}", tuple.0);
    println!("Second element: {}", tuple.1);
    println!("Third element: {}", tuple.2);

    println!("----------------------------------");

    //NOTE - 배열
    let array = [1, 2, 3, 4, 5];
    let array2 = [0; 5]; // 0으로 초기화된 길이 5의 배열
    println!("Array [1, 2, 3, 4, 5]: {:?}", array);
    println!("Array2 [0; 5]: {:?}", array2);

    println!("----------------------------------");

    //NOTE - 벡터
    let mut vector = vec![1, 2, 3];
    println!("Vector: {:?}", vector);
    vector.push(999);
    println!("Updated Vector: {:?}", vector);

    println!("----------------------------------");

    //NOTE - 디폴트 구현 및 구조체 맛보기
    let struct_example = StructExample {
        id: 1,
        name: "Example".to_string(),
    };
    println!("Struct Example: {:?}", struct_example);
    println!("Struct ID: {}, Name: {}", struct_example.id, struct_example.name);
    let default_struct = StructExample::default();
    println!("Default Struct Example: {:?}", default_struct);

    println!("----------------------------------");
}

#[derive(Debug, Default)]
struct StructExample {
    id: u32,
    name: String,
}