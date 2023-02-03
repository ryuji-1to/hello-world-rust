fn void() {}
fn return_void() {
    // implicit return >>> ()
    return ();
}
/// x plus y
///
/// # Examples
///
/// ```
/// let r = add(1,10);
/// assert_eq!(11,r);
/// ```
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
fn add_v2(x: i32, y: i32) -> i32 {
    x + y
}

fn change_string(x: &str) -> String {
    x.to_string()
}

enum Color {
    Red,
    Blue,
    Green,
    Hex(String),
}

// const HEX_STR: String = change_string("ffffff");
// const RED: Color = Color::Red;
// const HEX: Color = Color::Hex(HEX_STR);

fn always_error() -> Result<(), String> {
    Err("always error".to_string())
}

fn might_fail() -> Result<(), String> {
    let _result = always_error()?;
    Ok(())
}

fn fizz_buzz(value: i32) -> String {
    let result = if value % 15 == 0 {
        "fizz buzz".to_string()
    } else if value % 5 == 0 {
        "buzz".to_string()
    } else if value % 3 == 0 {
        "fizz".to_string()
    } else {
        value.to_string()
    };
    result
}

fn fizz_buzz_v2(value: i32) -> String {
    let result = match value {
        v if v % 15 == 0 => "fizz buzz".to_string(),
        v if v % 5 == 0 => "buzz".to_string(),
        v if v % 3 == 0 => "fizz".to_string(),
        _ => value.to_string(),
    };
    result
}

fn if_value() -> String {
    let mut result = "".to_string();
    if true {
        result = "hello".to_string();
    }
    result
}

fn string_to_color_token(value: &str) -> Option<Color> {
    match value {
        "red" => Some(Color::Red),
        "blue" => Some(Color::Blue),
        "green" => Some(Color::Green),
        _ => None,
    }
}

// macro
macro_rules! sum {
    ( $($x:expr),*)=>{
        {
            let mut result = 0;
            $(
            result = result + $x;
        )*
            result
        }
    };
}

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
    let void_ = void();
    println!("{:?}", void_);
    let void_ = return_void();
    println!("{:?}", void_);
    println!("{:?}", add(4, 5));
    println!("{:?}", add_v2(4, 9));
    let _f = if true { 10 } else { 20 };
    let _g = { 10 * 2 };
    let some: Result<&str, &str> = Ok("ok");
    println!("{:?}", some);
    let err: Result<&str, &str> = Err("error");
    println!("{:?}", err);
    let message = match might_fail() {
        Ok(_) => "処理に成功".to_string(),
        Err(cause_message) => cause_message,
    };
    println!("{:?}", message);
    // println!("before panic!");
    // panic!("hoge");
    // println!("after panic!");
    let input: Result<&str, &str> = Ok("test");
    let input = input.unwrap();
    println!("{:?}", input);
    println!("{}", sum![1, 2, 3, 4, 5]);
    println!("{}", fizz_buzz_v2(1));
    println!("{}", fizz_buzz_v2(3));
    println!("{}", fizz_buzz_v2(5));
    println!("{}", fizz_buzz_v2(15));
    let a = match string_to_color_token("red") {
        Some(a) => "found it".to_string(),
        None => "Nothing".to_string(),
    };
    println!("{:?}", a);
}
