pub fn example1() {
    println!("\nexampl1");
    let s = "abc".to_string();

    let a = &s;
    println!("{}", a);
}

// fn foo() -> &str {
//     let s = String::from("abc");
//     &s // 打脸
// }
fn foo() -> String {
    let s = String::from("abc");
    s
}

static ASTRING: &str = "uvw";

fn foo_static() -> &'static str {
    // 告诉编译器返回的引用具有static的lifetime
    let b = "agc";
    b
}

fn foo2(s: &str) -> &str {
    s
}

pub fn example2() {
    println!("\nexampl2");
    let a = foo();

    let s1 = foo_static();
    println!("static s1: {}", s1);

    let x = String::from("abc");
    let y = "123456";
    let s = longest(&x, y);
    println!("longest is {}", s);

    let s2 = String::from("xyz");
    let result = foo2(&s2);
    println!("s is {}", result);
}

fn longest<'x>(x: &'x str, y: &'x str) -> &'x str {
    // 告诉编译器, 返回的引用具有和入参相同的lifetime
    if x.len() > y.len() {
        x
    } else {
        let b = "yangkai";
        return b;
    }
}

pub fn example3() {
    println!("\nexampl3");
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is \"{}\"", result);
    }
}

pub fn example4() {
    println!("\nexampl4");
    let string1 = String::from("long string is long");
    let result; // declare result before the inner scope

    {
        let string2 = String::from("xyz");
        let s = longest(string1.as_str(), string2.as_str()); // assign value inside the inner scope
        result = s.to_string(); // 要想放到外面的result去, 必须这么干
    }

    println!("The longest string is \"{}\"", result);
}
