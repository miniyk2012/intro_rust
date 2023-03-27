use std::collections::HashMap;

fn foo(s: &String) {
    println!("{}", s)
}

fn demo1() {
    println!("\ndemo1");
    let s = "a super man".to_string();
    for i in 1..10 {
        foo(&s);
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    nickname: String,
}

fn demo2() {
    println!("\ndemo2");
    let person = Person {
        name: String::from("Alice"),
        nickname: String::from("Ali"),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref nickname } = person;

    println!("The person's age is {}", nickname);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    // println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.nickname` can be used as it is not moved
    println!("The person's age from person struct is {}", person.nickname);
}

fn demo3() {
    println!("\ndemo3");

    let s1 = String::from("a superman");
    let s2 = String::from("two superman");
    let s3 = String::from("3 superman");
    let s4 = String::from("four superman");

    let v = vec![s1, s2, s3, s4];
    // println!("{:?}", s1);

    println!("{:?}", v);

    // let a = v[0]; // 不能move出来
    let a = &v[0];
    println!("{:?}", a);
}

fn demo4() {
    println!("\ndemo4");
}

#[derive(Debug)]
struct User {
    name: String,
    nickname: Option<String>,
    age: u32,
}

fn demo5() {
    let mut user = User {
        name: "yang".to_string(),
        nickname: Some("hello".to_string()),
        age: 34,
    };
    if let Some(nickname) = user.nickname.take() {
        println!("{:?}", nickname);
    }

    println!("{:?}", user);
}

fn demo6() {}
pub fn main() {
    demo1();
    demo2();
    demo3();
    demo4();
    demo5();
}
