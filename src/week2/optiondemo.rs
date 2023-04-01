#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn optionDemo1() {
    println!("\noptiondemo1");
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let a = UsState::Alabama;
    let coin = Coin::Quarter(a);
    let mut count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    let mut count = 0;
    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("coin={:?}", coin);
}

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
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
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

pub fn enumDemo() {
    println!("\nenumDemo");
    let a = WebEvent::PageLoad;
    let b = WebEvent::KeyPress('w');
    let c = WebEvent::Paste(String::from("hello world"));
    let d = WebEvent::Click { x: 2, y: 3 };
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let add = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let sub = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;
    println!("add={}", add.run(1, 3));
    println!("sub={}", sub.run(1, 3));
}

pub fn matchDemo1() {
    println!("\nmatchDemo1");
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("binary={}", binary);
}

pub fn whileLet() {
    println!("\nwhileLet");
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

struct User {
    name: String,
    age: u32,
    student: bool,
}

pub fn matchStruct() {
    println!("\nmatchStruct");
    let a = User {
        name: String::from("yangkai"),
        age: 21u32,
        student: true,
    };
    let User { name, age, student } = a;
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
}

fn foo((a, b, c): (u32, u32, char)) {
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}
fn foo2(User { name, age, student }: User) {
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
}

pub fn matchFun() {
    println!("\nmatchFun");
    let a = (1, 2, 'a');
    foo(a);
    let a = User {
        name: String::from("yangkai"),
        age: 21u32,
        student: true,
    };
    foo2(a);

    let l: [u32; 3] = [1; 3];
    assert_eq!(Some(&1), l.last());
    assert_eq!(Some(&1), l.first());
    assert_eq!(3, l.len());

    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    // This loop prints: 0 1 2
    for x in array {
        println!("{x}");
    }
}

fn option1() -> Option<i32> {
    Some(100)
}

struct Person {
    name: String,
}

impl Person {
    fn eat(self) {
        println!("{} is eating", self.name);
        // self is consumed here
    }
}

pub fn consumeSelf() {
    let person = Person {
        name: "Alice".to_string(),
    };
    person.eat(); // person is consumed here

    // person.eat();
    // This will not compile because person has been consumed

    let a = Some(String::from("abc"));
    match a {
        Some(ref x) => println!("a={:?}", x),
        _ => println!("none"),
    }
    println!("{:?}", a.unwrap());
}
