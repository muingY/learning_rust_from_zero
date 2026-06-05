pub fn run() {
    /*
     * for_each와 유사.
     * for_each는 리턴값이 없이 종결, map은 반복자를 다시 리턴.
     * -> 루프를 돌려서 값들을 변경하고, 변경된 값을 이용해서 다시 무언가를 할 때 자주 사용할 수 있음.
     */
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().map(|&x| x + 1).collect();
    println!("v  : {:?}", v);
    println!("v1 : {:?}", v1);

    println!("----------------------------------");

    /*
     * filter는 괄호 안에 있는 클로저 조건을 만족하는 원소들로만 재구성된 iterator를 리턴함.
     */
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect(); // 짝수 필터링
    println!("v  : {:?}", v);
    println!("v1 : {:?}", v1);

    /*
     * filter_map
     */
    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<_> = a.iter().map(|s| s.parse::<i32>()).filter(|s| s.is_ok()).map(|s| s.unwrap()).collect();
    println!("a : {:?}", a);
    println!("v : {:?}", v);
    let v: Vec<_> = a.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("a : {:?}", a);
    println!("v : {:?}", v);

    /*
     * for_each
     */
    let mut v = vec![1, 2, 3, 4, 5];
    println!("original v : {:?}", v);
    v.iter_mut().for_each(|x| *x *= 10);
    println!("after for_each v : {:?}", v);

    /*
     * take_while은 filter와 유사하게 클로저의 조건에 만족하는 원소들만 추려서 iterator를 만든다.
     * filter는 조건을 모든 것을 원소 끝까지 탐색하는 반면, take_while은 조건을 벗어나는 원소를 처음 만나면 더이상 찾지 않는다.
     */
    let v = vec![1, 3, 5, 7, 9, 10, 11, 13, 15];
    let v1: Vec<_> = v.iter().filter(|x| *x % 2 == 1).collect(); // 모든 홀수를 찾는다.
    let v2: Vec<_> = v.iter().take_while(|x| *x % 2 == 1).collect(); // 홀수가 아닐 때까지의 홀수만
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    println!("----------------------------------");
}