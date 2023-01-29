fn main() {
    let message = "hoge";
    for _i in 0..3 {
        println!("{}", message);
    }
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
    // String type
    let _message_2 = String::from("hello world");
    // String slice
    let _message_3 = "hello world";
    let _message_string = _message_3.to_string();
    let mut message_push = String::from("hello");
    message_push.push_str(", world!");
    println!("{:?}", message_push);
    // hash map
    let mut scores = std::collections::HashMap::new();
    scores.insert("Sato", 100);
    scores.insert("Tanaka", 90);
    scores.entry("Tanaka").or_insert(100);
    let solar_distance = std::collections::HashMap::from([("Mercury", 0.4), ("Venus", 0.7)]);
    println!("{:?}", solar_distance);
}
