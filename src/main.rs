fn main() {
    let message = "hoge";
    // if true {
    //     message = "hello, foo";
    // } else {
    //     message = "hello, hoge";
    // }
    const _URL: &str = "https://example.com";
    let _a: i8 = 127;
    let _bool: bool = true;
    let __bool = false;
    let _c = 'c';
    let _d = "d";
    let e = ("hoge", 128);
    // array is fixed length
    let this_is_array = ["hoge", "hoge", "fuga"];
    let mut v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];
    v.push(99);
    println!("{}", e.0);
    println!("{}", this_is_array[0]);
    println!("{:?}", v);
    println!("{:?}", _v);

    for _i in 0..3 {
        println!("{}", message);
    }
}
