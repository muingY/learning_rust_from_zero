pub fn run() {
    // 일반 문자열은 String 타입이 아닌 &str 타입.
    // static 데이터로, 프로그램 실행되는 동안 유지되는 값.
    // 즉, 읽을 때는 주소값으로만 읽어야 하므로, 대여(borrow) 형태로 사용해야한다는 의미.
    // = &str 형태인 이유
    let s: &str = "Hello, World!";
    println!("s: {}", s);

    // rust의 str은 utf-8 인코딩을 사용.
    let s = "hello";
    println!("s: {} (len = {})", s, s.len());
    let s = "안녕하세요";
    println!("s: {} (len = {})", s, s.len());

    // 실제 문자 개수를 구하려면 char로 변형해서 사용.
    println!("s: {} (chars count = {})", s, s.chars().count());

    let s = "hello";
    // println!("{}", s[0]);  // <- 인덱싱 접근 불가. 바이트 단위가 아닌 1~4 바이트이기 때문에 막음.
    // println!("{}", &s[1]);
    println!("s[0..1]: {}", &s[0..1]);  // <- 슬라이싱은 가능. 0~1 바이트 범위는 'h'에 해당하므로 정상 출력.
    println!("s[1..4]: {}", &s[1..4]);  // <- 1~4 바이트 범위는 'e', 'l', 'l'에

    let s = "안녕하세요";
    println!("s[0..3]: {}", &s[0..3]);  // <- 0~3 바이트 범위는 '안'에 해당하므로 정상 출력.
    
    let mut bytes = "bors".bytes();
    println!("{}", bytes.next().unwrap());
    println!("{}", bytes.next().unwrap());
    println!("{}", bytes.next().unwrap());
    println!("{}", bytes.next().unwrap());

    let bananas = "bananas";
    println!("{}", bananas);
    println!("banana contain \"ana\" : {}", bananas.contains("ana"));
    println!("banana contain \"asdf\" : {}", bananas.contains("asdf"));

    let text = "foo\r\nbar\r\nbaz\r";
    let mut lines = text.lines();
    println!("{}", lines.next().unwrap());
    println!("{}", lines.next().unwrap());
    println!("{}", lines.next().unwrap());

    let text = "This is RUST";
    let mut iter = text.split_whitespace();
    println!("{}", iter.next().unwrap());
    println!("{}", iter.next().unwrap());
    println!("{}", iter.next().unwrap());

    let text_with_whitespace = "    HI!   ";
    println!("_{}_", text_with_whitespace);
    println!("after trim : _{}_", text_with_whitespace.trim());

    let text_uppercase = "AHJFLJSLKJF";
    println!("{}", text_uppercase);
    println!("after to_lowercase : {}", text_uppercase.to_lowercase());
    
    println!("----------------------------------");
}