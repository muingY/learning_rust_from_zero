pub fn run() {
    /*
     * - iter() : 소유권 이동되지 않음. iter 이후 해당 컬렉션 사용 가능.
     * - into_iter() : 소유권이 넘어감.
     * - iter_mut() : 소유권 넘어가지 않고, mut 레퍼런스로 받음. 컬렉션의 값을 수정할 수 있음.
     */
    let v = vec![1, 2, 3, 4, 5];
    for val in v.iter() {
        println!("{}", val);
    }
    println!("");
    for val in &v { // &v로 사용해도 벡터의 iterator가 자동으로 사용됨. (= v.iter())
        println!("{}", val);
    }

    let v = vec![1, 2, 3, 4, 5];
    for val in v.into_iter() {
        println!("{}", val);
    }
    // println!("v len : {}", v.len()); // <- 소유권 에러 발생. into_iter()에 의해 소유권이 넘어갔기 때문.

    let mut v = vec![1, 2, 3, 4, 5];
    println!("original v : {:?}", v);
    for x in v.iter_mut() {
        *x *= 10;
    }
    println!("after iter_mut : {:?}", v);

    v.iter_mut().for_each(|x| *x += 1); // <- for_each(클로저) 사용도 가능.
    println!("after iter_mut().for_each(...) : {:?}", v);

    println!("----------------------------------");
}