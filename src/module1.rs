use std::collections::HashMap;

fn module1demo() {
    println!("module1demo");
    demo1();
    demo3();
    demo4();
    demo5();
    demo6();
    demo7();
    demo8();
    demo9();
    demo10();
    demo11();
    demo12();
}

fn demo1() {
    println!("demo1");
    let s = String::from("Hello world"); // 字符串尺寸不固定, 因此用String::from把字面量放到变量里, 以后长度可以变化
    println!("{s}");
    println!("Hello, world!");
    let _a: u64 = 1_000_000_000_000;
    let _b = 0xff;
    let _c = 0b1111_1111;
    let d: char = '杨';
    println!("{d}");
    let e = b'A';
    println!("{e}");
    let f = 100u32; // 固定尺寸, 空间不会变
    println!("{f}");
}

//  fn demo2() {
//     let hello = String::from("你好");
//     let a = hello[0];  // 会报错, 字符串不能用索引
//     println!("{a}");
//  }

fn demo3() {
    println!("\ndemo3");
    let unicode_codepoint = "\u{00e9}"; // 可以用来写表情
    println!("{unicode_codepoint}");
    let s = r###"
hello #world"""##
    "###;
    println!("{s}");
    let s2 = r"\u{009e}"; // 禁止转义
    println!("{s2}");
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("{:?}", bytestring)
}

fn demo4() {
    println!("\ndemo4");
    let arr: [i32; 5] = [1, 2, 3, 4, 4]; // 数组大小是不能变化的
    println!("{:?}", arr);
    println!("{}", arr[2]); // 数组可以用索引
                            // let b = arr[5];
                            // println!("{}", b);  // 会在编译期就报错, 而非运行期
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let c = &months[0][2..3];
    println!("{}", c);
    // println!("{}", months[12]);  // Compiling error: this operation will panic at runtime, 编译期报错
}

fn demo5() {
    println!("\ndemo5 动态数组");
    let v: Vec<i32> = Vec::new();

    println!("{:?}", v);
    let v: Vec<u32> = vec![2, 3, 4]; // 重新定义了v
    println!("{:?}", v);
    // v = Vec::new();  // cannot assign twice to immutable variable `v`
    // let  v:  Vec<u8> = Vec::new();
    let mut v: Vec<u8> = Vec::new(); // cannot borrow `v` as mutable, as it is not declared as mutable
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);
    // println!("{}", v[3]);  //这里会 runtime panic: thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 3', src/main.rs:71:20
}

fn demo6() {
    println!("\ndemo6 hash map");
    let mut scores: HashMap<String, i32> = HashMap::<String, i32>::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Yellow"), 10);
    // scores.insert("a", 200);
    println!("{:#?}", scores);
}

fn demo7() {
    println!("\ndemo7 tuple");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    let a = tup.0;
    let b = tup.1;
    let c = tup.1;
    println!("{} {} {}", a, b, c);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    age: u64,
}

fn demo8() {
    println!("\ndemo8 struct");
    let a = User {
        active: true,
        username: String::from("yangkai"),
        email: String::from("mike@rustcc.cn"),
        age: 50,
    };
    println!("{:#?}", a);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn demo9() {
    println!("\ndemo9 枚举");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);
    println!("{:?}", six);
}

fn demo10() {
    println!("\ndemo10 if-else");
    let a = 1;
    // 表达式返回值, 语句不返回值
    let b = if a == 1 { 10 } else { 20 };
    println!("b={}", b);
    if a == 1 {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn demo11() {
    println!("\ndemo11 循环 loop, while, for, range");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break具有把值返回的语义, 因此可以加分号
        }
    };
    println!("the result={result}");

    let mut a: usize = 1;
    while a < 10 {
        println!("the result is {a}");
        a += 1;
    }

    let z = loop {
        a -= 1;
        println!("the result is {a}");
        if a == 5 {
            break a;
        }
    };
    println!("z's value is {:?}", z);

    let a = [10, 20, 30, 40, 50];
    // for是用来迭代的, 不干其他事
    for ele in a {
        println!("the value is {ele}");
    }

    let r = 1..=4;
    for number in r {
        println!("{number}")
    }
    println!();
    let r = 1..=4;
    for number in r.rev() {
        println!("{number}")
    }
    println!();
    let r = 1..=4;
    for number in 'A'..'b' {
        println!("{number}")
    }
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let r = 1..=3;
    println!("{:?}", &a[r]);
}

fn foo(n: u16) -> u16 {
    return n * 2;
}
fn demo12() {
    println!("\ndemo12 函数");
    let n: u8 = 10;
    println!("{}", foo(n.into()));
    println!("{}", foo(n as u16));
}
