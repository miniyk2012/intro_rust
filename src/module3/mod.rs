#[derive(Debug)]
struct User1 {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32,
}

fn demo0() {
    println!("demo0");
    let mut user1 = User1 {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{:?}", user1);

    let active = true;
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");
    let user2 = User1 {
        active,
        username,
        email,
        sign_in_count: 3,
    };
    println!("{:?}", user2);

    let user3 = User1 {
        sign_in_count: 30,
        email: String::from("yangkai@example.com"),
        ..user1
    };
    println!("{:?}", user3);
}

#[derive(Debug)]
struct ArticleModule;
fn demo1() {
    println!("\ndemo1 单元结构体");
    let module = ArticleModule;
    println!("{:?}", module);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn arean(&self, n: u32) -> u32 {
        self.width * self.height * n
    }
    fn width(&self) -> u32 {
        self.width
    }
    // static associate method
    fn foo(n: u32) -> u32 {
        100
    }

    // 行业约定
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn demo2() {
    println!("\ndemo2");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = &&rect1;

    println!(
        "The n times area of the rectangle is {} square pixels.",
        rect1.arean(5)
    );
    println!(
        "The n times area of the rectangle is {} square pixels.",
        rect2.arean(4)
    );
}

fn demo3() {
    println!("\ndemo3");
    let rect = Rectangle::new(5, 8);
    Rectangle::foo(10);
    println!("{:?}", rect)
}

// Newtype模式

struct NewType(Vec<u8>);

// impl i8 {

// }

#[derive(Debug)]
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}
fn demo4() {
    println!("\ndemo4");
    let a = WebEvent::PageLoad;
    let b = WebEvent::PageUnload;
    let c = WebEvent::KeyPress('c');
    let d = WebEvent::Paste(String::from("batman"));
    let e = WebEvent::Click { x: 320, y: 240 };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
    println!("{:?}", e);
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn demo5() {
    println!("\ndemo5");
    let add = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    println!("{}", add.run(100, 200))
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn demo6() {
    println!("\ndemo6");
    let a = UsState::Alabama;
    let b = Coin::Quarter(a);
    let r = value_in_cents(b);

    println!("{}", r);
    let c = UsState::Alabama;
    println!("{:?}", c);
}
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    student: bool,
}

fn demo7() {
    println!("\ndemo7");

    let a = User {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    let User {
        ref name,
        age,
        student,
    } = a;

    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
    println!("{:?}", a);
}

// Copy是指长度固定的类型
fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0]; // 先取[], 再取&

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn demo8() {
    println!("\ndemo8");
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    println!("{:?}", char_list);
}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn demo9() {
    println!("\ndemo9");
    let integer = Point { x: 5u32, y: 10 };
    println!("{:?}", integer);
    let float: Point<f32> = Point { x: 1.0, y: 4.0 };
    println!("{:?}", float);
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn demo10() {
    println!("\ndemo10");
    let p: Point<f32> = Point { x: 5f32, y: 10f32 };

    println!("p.x = {}", p.x());
    println!("{}", p.distance_from_origin())
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn demo11() {
    println!("\ndemo11");
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

pub fn main() {
    demo0();
    demo1();
    demo2();
    demo3();
    demo4();
    demo5();
    demo6();
    demo7();
    demo8();
    demo9();
    demo10();
    demo11();
}
