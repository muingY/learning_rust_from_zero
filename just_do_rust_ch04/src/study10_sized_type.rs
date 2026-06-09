/*
 * ?Sized 의미
 * : 꼭 Sized 트레잇을 구현한 타입이 아니어도 된다.
 * 
 * 각 타입마다 그 크기를 알 수 있게끔 Sized 트레잇을 구현함.
 * 따라서, 해당 객체의 타입이 Sized를 구현한 객체이면, 컴파일 타임에 사이즈를 알 수 있는 객체.
 * 
 * Rust는 모든 변수에 대해서 컴파일 타임에 그 크기를 한정할 수 있어야 한다.
 * fn test<T> (t: T)
 * = fn test<T: Sized>(t: T) 와 같이 취급
 */

pub fn run() {
    struct _Foo<T: ?Sized>(T);

    struct _FooUse1(_Foo<i32>);
    struct _FooUse2(_Foo<[i32]>); // [i32] 배열이므로, 크기를 한정할 수 없음.
    // struct Foo<T: ?Sized>(T) 로 제너릭 T에 대해 ?Sized 를 붙이면 가능.

    let color_a: Rgba = (0.0, 0.0, 0.0, 1.0);
    let color_b: Rgba = (1.0, 1.0, 1.0, 1.0);

    println!("blend color = {:?}", blend_color(color_a, color_b));
}

type Rgba = (f64, f64, f64, f64);

fn blend_color(_a: Rgba, _b: Rgba) -> Rgba {
    return (0.24, 0.0, 0.0, 1.0);
}