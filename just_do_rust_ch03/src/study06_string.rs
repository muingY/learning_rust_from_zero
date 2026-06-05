pub fn run() {
    // &str은 고정된 문자열, String은 가변 크기 문자열을 다룸.
    // &str -> 프로그램 바이너리에 있던 문자열에 메모리로 옮겨지면, 그 메모리 주소를 가리키는 레퍼런스 변수
    // String -> 힙에 저장되는 문자열 값에 대한 소유권을 가진 변수 (&String 형태로 대여해줄수도 있음)
    let hello = String::from("Hello, World!");
    println!("{}", hello);

    let _ = "Hello, ".to_owned(); // to_owned()가 좀 더 가벼움. &str 타입을 위한 용도.
    let _ = "world!".to_string(); // u32 등 여러 타입들에 대해서도 string으로 변형하기 위해 사용되는 범용 메서드이기 떄문에 조금 더 무거움.

    let a = String::from("foo");
    let b = String::from("bar");

    let c = a.clone() + &b;
    println!("{} + {} = {}", a, b, c);

    // format! 매크로 사용
    let c = format!("{}{}", &a, &b);
    println!("{} + {} = {}", a, b, c);
    let c = format!("{}{}", "aaa", "bbb");
    println!("aaa + bbb = {}", c);

    let mut binding = String::from("hello");
    let s= binding.as_bytes();
    println!("{} -> as_bytes -> {:?}", binding, s);

    println!("{} (len: {})", binding, binding.len());
    binding.clear();
    println!("after clear() = {} (len: {})", binding, binding.len());

    let mut string = String::from("Hello, String");
    println!("original string : {}", string);
    string.insert(2, '!');
    println!("insert(2, \'!\') : {}", string);
    string.push('?');
    println!("push('?') : {}", string);
    string.push_str("_end push!");
    println!("push_str(\"_end push\") : {}", string);
    string.remove(2);
    println!("remove(2) : {}", string);

    string.truncate(2);
    println!("truncate(2) : {}", string);

    println!("----------------------------------")
}