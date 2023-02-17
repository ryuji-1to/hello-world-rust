mod handlers;
mod repositories;

use crate::repositories::{TodoRepository, TodoRepositoryForMemory};
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use handlers::{all_todo, create_todo, delete_todo, find_todo, update_todo};
use std::net::SocketAddr;
use std::{env, sync::Arc};

async fn root() -> &'static str {
    "Hello World!!!"
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>).get(all_todo::<T>))
        .route(
            "/todos/:id",
            get(find_todo::<T>)
                .delete(delete_todo::<T>)
                .patch(update_todo::<T>),
        )
        .layer(Extension(Arc::new(repository)))
}

#[tokio::main]
async fn main() {
    // logging initiallize
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repositories::{CreateTodo, Todo};
    use axum::response::Response;
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
    };
    use tower::ServiceExt;

    fn build_todo_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body))
            .unwrap()
    }

    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_todo(res: Response) -> Todo {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Todo = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body:{}", body));
        todo
    }

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let repository = TodoRepositoryForMemory::new();
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello World!!!");
    }
}

// #[tokio::test]
// async fn should_return_user_data() {
//     let req = Request::builder()
//         .uri("/users")
//         .method(Method::POST)
//         .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
//         .body(Body::from(r#"{"username":"田中"}"#))
//         .unwrap();
//     let repository = TodoRepositoryForMemory::new();
//     let res = create_app(repository).oneshot(req).await.unwrap();
//     let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
//     let body: String = String::from_utf8(bytes.to_vec()).unwrap();
//     let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");
//     assert_eq!(
//         user,
//         User {
//             id: 1337,
//             username: "田中".to_string()
//         }
//     )
// }
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct CreateUser {
//     username: String,
// }
//
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct User {
//     id: u64,
//     username: String,
// }

// async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };
//
//     (StatusCode::CREATED, Json(user))
// }

// use anyhow::{Context, Result};
// use core::fmt::Display;
// use std::cmp::{Eq, PartialEq};
// use std::fmt;
// use std::fs::File;
// use std::ops::Add;
// use thiserror::Error;
// mod foo;
//
// mod parent {
//     pub mod child {
//         pub fn print_hoge() {
//             println!("hoge");
//         }
//     }
//     mod private_child {
//         pub fn print_hoge() {
//             println!("hoge");
//         }
//     }
// }
//
// #[derive(Debug, Error)]
// enum ApiError {
//     InternalServerError(String),
//     NotFound,
// }
//
// impl fmt::Display for ApiError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "[ApiError]")
//     }
// }
//
// impl error::Error for ApiError {}
//
// fn fetch_api() -> Result<(), ApiError> {
//     Err(ApiError::InternalServerError(
//         "[always_my_error]".to_string(),
//     ))
// }
//
// fn maybe_fail() -> Result<(), Box<dyn error::Error>> {
//     let _r = fetch_api()?;
//     let _f = File::open("hoge.txt")?;
//     Ok(())
// }
//
// fn treat_error() -> Result<(), Box<dyn error::Error>> {
//     let _l = maybe_fail()?;
//     Ok(())
// }
//
// fn void() {}
// fn return_void() {
//     // implicit return >>> ()
//     return ();
// }
// /// x plus y
// ///
// /// # Examples
// ///
// /// ```
// /// let r = add(1,10);
// /// assert_eq!(11,r);
// /// ```
// fn add(x: i32, y: i32) -> i32 {
//     return x + y;
// }
// fn add_v2(x: i32, y: i32) -> i32 {
//     x + y
// }
//
// fn change_string(x: &str) -> String {
//     x.to_string()
// }
//
// enum Color {
//     Red,
//     Blue,
//     Green,
//     Hex(String),
// }
//
// // const HEX_STR: String = change_string("ffffff");
// // const RED: Color = Color::Red;
// // const HEX: Color = Color::Hex(HEX_STR);
//
// fn always_error() -> Result<(), String> {
//     Err("always error".to_string())
// }
//
// fn might_fail() -> Result<(), String> {
//     let _result = always_error()?;
//     Ok(())
// }
//
// fn fizz_buzz(value: i32) -> String {
//     let result = if value % 15 == 0 {
//         "fizz buzz".to_string()
//     } else if value % 5 == 0 {
//         "buzz".to_string()
//     } else if value % 3 == 0 {
//         "fizz".to_string()
//     } else {
//         value.to_string()
//     };
//     result
// }
//
// fn fizz_buzz_v2(value: i32) -> String {
//     let result = match value {
//         v if v % 15 == 0 => "fizz buzz".to_string(),
//         v if v % 5 == 0 => "buzz".to_string(),
//         v if v % 3 == 0 => "fizz".to_string(),
//         _ => value.to_string(),
//     };
//     result
// }
//
// fn if_value() -> String {
//     let mut result = "".to_string();
//     if true {
//         result = "hello".to_string();
//     }
//     result
// }
//
// fn string_to_color_token(value: &str) -> Option<Color> {
//     match value {
//         "red" => Some(Color::Red),
//         "blue" => Some(Color::Blue),
//         "green" => Some(Color::Green),
//         _ => None,
//     }
// }
//
// fn add_until(start: i32, end: i32) -> i32 {
//     let mut sum = 0;
//     let mut temp = start;
//     loop {
//         sum += temp;
//         if temp == end {
//             break sum;
//         }
//         temp += 1;
//     }
// }
//
// fn add_until_while(start: i32, end: i32) -> i32 {
//     let mut sum = 0;
//     let mut temp = start;
//     while temp <= end {
//         sum += temp;
//         temp += 1;
//     }
//     sum
// }
//
// // macro
// macro_rules! sum {
//     ( $($x:expr),*)=>{
//         {
//             let mut result = 0;
//             $(
//             result = result + $x;
//         )*
//             result
//         }
//     };
// }
//
// // clone method
// #[derive(Clone)]
// struct Line(i32, i32);
//
// fn hello_print(message: String) {
//     println!("hello {}", message);
// }
//
// fn hello_print_v2(message: &str) {
//     println!("hello {}", message);
// }
//
// fn print(value: i32) {
//     println!("{}", value);
// }
//
// // just take shoyuken function
// fn take<T>(_value: T) {}
//
// fn fizz(value: i32) -> String {
//     let result = if value % 3 == 0 {
//         String::from("fizz")
//     } else {
//         format!("{}", value)
//     };
//     // clone if you use result
//     let clone_result = result.clone();
//     take(clone_result);
//     result
// }
//
// fn hello() -> &'static str {
//     "hello"
// }
//
// // lifetime
// fn life_time() {
//     println!("{}", hello());
// }
//
// // heap
// fn heap() {
//     let a = Box::new("test");
//     println!("{}", a);
// }
//
// // structure
//
// struct User {
//     name: String,
//     age: i32,
// }
//
// struct StraightLine(i32, i32);
//
// #[derive(Debug, PartialEq, Eq)]
// struct Fraction(u32, u32);
//
// impl Fraction {
//     // numerator 分子, denominator: 分母
//     fn new(numerator: u32, denominator: u32) -> Self {
//         let gcf_value = Self::gcf(numerator, denominator);
//         Self(numerator / gcf_value, denominator / gcf_value)
//     }
//
//     fn gcf(value1: u32, value2: u32) -> u32 {
//         let (mut a, mut b) = if value1 > value2 {
//             (value1, value2)
//         } else {
//             (value2, value1)
//         };
//         let mut r = a % b;
//         while r != 0 {
//             a = b;
//             b = r;
//             r = a % b;
//         }
//         b
//     }
// }
//
// impl Add for Fraction {
//     // 関連型
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         let lcm = self.1 / Self::gcf(self.1, other.1) * other.1;
//
//         let a = self.0 * (lcm / self.1);
//         let b = other.0 * (lcm / other.1);
//         Fraction::new(a + b, lcm)
//     }
// }
//
// impl Display for Fraction {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}/{}", &self.0, &self.1)
//     }
// }
//
// impl Display for User {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "user name is {}, age is {}", &self.name, &self.age)
//     }
// }
// // Self implement User
// impl User {
//     fn new(name: String, age: i32) -> Self {
//         Self { name, age }
//     }
//
//     fn description(&self) -> String {
//         format!("user name is {}, age is {}", &self.name, &self.age)
//     }
//
//     fn rename(&mut self, name: String) {
//         self.name = name;
//     }
// }
//
// trait Area {
//     fn area(&self) -> u32;
// }
//
// struct Square(u32);
//
// impl Area for Square {
//     fn area(&self) -> u32 {
//         self.0.pow(2)
//     }
// }
//
// impl Square {
//     fn new(side: u32) -> Self {
//         Self(side)
//     }
// }
//
// fn calc_square() {
//     let my_square = Square::new(5);
//     println!("area is {}", my_square.area());
// }
//
// fn structure() {
//     let name = "sato".to_string();
//     let age = 30;
//     let user = User { name, age };
//     println!("name is {}, age is {}", user.name, user.age);
// }
//
// fn class() {
//     let mut user = User::new(String::from("sato"), 20);
//     user.rename(String::from("kato"));
//     println!("{}", user.description());
//     println!("name is {}", user.name);
//     println!("{}", user);
// }
//
// fn fraction() {
//     let a = Fraction::new(8, 18);
//     println!("{}", a);
// }
//
// // #[derive(PartialEq, Eq)]
// // struct Point {
// //     value: f64,
// // }
//
// fn main() {
//     let message = "hoge";
//     for _i in 0..3 {
//         println!("{}", message);
//     }
//     // if true {
//     //     message = "hello, foo";
//     // } else {
//     //     message = "hello, hoge";
//     // }
//     const _URL: &str = "https://example.com";
//     let _a: i8 = 127;
//     let _bool: bool = true;
//     let __bool = false;
//     let _c = 'c';
//     let _d = "d";
//     let e = ("hoge", 128);
//     // array is fixed length
//     let this_is_array = ["hoge", "hoge", "fuga"];
//     let mut v: Vec<i32> = Vec::new();
//     let _v = vec![1, 2, 3];
//     v.push(99);
//     println!("{}", e.0);
//     println!("{}", this_is_array[0]);
//     println!("{:?}", v);
//     println!("{:?}", _v);
//     // String type
//     let _message_2 = String::from("hello world");
//     // String slice
//     let _message_3 = "hello world";
//     let _message_string = _message_3.to_string();
//     let mut message_push = String::from("hello");
//     message_push.push_str(", world!");
//     println!("{:?}", message_push);
//     // hash map
//     let mut scores = std::collections::HashMap::new();
//     scores.insert("Sato", 100);
//     scores.insert("Tanaka", 90);
//     scores.entry("Tanaka").or_insert(100);
//     let solar_distance = std::collections::HashMap::from([("Mercury", 0.4), ("Venus", 0.7)]);
//     println!("{:?}", solar_distance);
//     let void_ = void();
//     println!("{:?}", void_);
//     let void_ = return_void();
//     println!("{:?}", void_);
//     println!("{:?}", add(4, 5));
//     println!("{:?}", add_v2(4, 9));
//     let _f = if true { 10 } else { 20 };
//     let _g = { 10 * 2 };
//     let some: Result<&str, &str> = Ok("ok");
//     println!("{:?}", some);
//     let err: Result<&str, &str> = Err("error");
//     println!("{:?}", err);
//     let message = match might_fail() {
//         Ok(_) => "処理に成功".to_string(),
//         Err(cause_message) => cause_message,
//     };
//     println!("{:?}", message);
//     // println!("before panic!");
//     // panic!("hoge");
//     // println!("after panic!");
//     let input: Result<&str, &str> = Ok("test");
//     let input = input.unwrap();
//     println!("{:?}", input);
//     println!("{}", sum![1, 2, 3, 4, 5]);
//     println!("{}", fizz_buzz_v2(1));
//     println!("{}", fizz_buzz_v2(3));
//     println!("{}", fizz_buzz_v2(5));
//     println!("{}", fizz_buzz_v2(15));
//     let a = match string_to_color_token("red") {
//         Some(a) => "found it".to_string(),
//         None => "Nothing".to_string(),
//     };
//     println!("{:?}", a);
//     if let Some(color) = string_to_color_token("red") {
//         println!("red");
//     }
//     let loop_result = add_until(1, 3);
//     println!("{}", loop_result);
//     let scores = vec![100, 30, 80, 70, 95];
//     for score in scores.iter() {
//         println!("score is {}", score);
//     }
//     // syoyuken
//     let a = String::from("hoge");
//     let b = a;
//     println!("{}", b);
//     let c = 100;
//     let d = c;
//     println!("{}", c);
//     // move
//     let world = String::from("world");
//     // move to hello_print
//     hello_print(world);
//     // error boooom!!!
//     // println!("main print {}", world);
//     let world_v2 = String::from("world");
//     hello_print_v2(&world_v2);
//     println!("main print {}", world_v2);
//     let e = 999;
//     let f = &e;
//     print(*f);
//     println!("1 {}", fizz(1));
//     println!("1 {}", fizz(3));
//     life_time();
//     heap();
//     structure();
//     class();
//     calc_square();
//     fraction();
//     let f_a = Fraction::new(8, 18);
//     let f_b = Fraction::new(1, 2);
//     println!("fraction {}", f_a + f_b);
//     let nan = f64::NAN;
//     assert!(nan != nan);
//     // let p1 = Point { value: f64::NAN };
//     // assert!(p1 == p1);
//     use crate::parent::child::print_hoge;
//     print_hoge();
//     foo::bar::baz();
//     treat_error();
// }
//
// // test
//
// #[derive(Debug, PartialEq, Eq)]
// struct Book {
//     name: String,
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn add_test() {
//         assert_eq!(2, add(1, 1));
//     }
//
//     #[test]
//     fn book_test() {
//         let book1 = Book {
//             name: "book name".to_string(),
//         };
//         let book2 = Book {
//             name: "book name".to_string(),
//         };
//         assert_eq!(book1, book2);
//     }
//
//     #[test]
//     fn return_result_test() -> Result<(), ()> {
//         Ok(())
//     }
// }
//
