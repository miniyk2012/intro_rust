fn foo() -> Option<()> {
    Some(())
}

fn demo1() {
    println!("\ndemo1");
    let _a = foo();
    let a = None;
    println!("{:?}", a);
    // let b = a.unwrap();
    // println!("{:?}", b);
    let b: u32 = a.unwrap_or(12);
    println!("b={:?}", b);
    let _x = "abc";
}

fn foo2(s: &mut [u32]) {
    s[1] = 4;
}
fn demo2() {
    println!("\ndemo2");
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let s = &mut v[1..9];
    foo2(s);
    println!("{:?}", v)
}

use std::str;
fn demo3() {
    println!("\ndemo3");
    // some bytes, in a stack-allocated array
    let sparkle_heart = [240, 159, 146, 150];

    // We know these bytes are valid, so just use `unwrap()`.
    let sparkle_heart: &str = str::from_utf8(&sparkle_heart).unwrap();

    assert_eq!("💖", sparkle_heart);
}

fn demo4() {
    println!("\ndemo4");
    let a = String::from("i am a superman");
    let a = "i am a superman".to_string();
    let a = "i am a superman".to_owned();
    let b: &str = &a[..];
    let b = a.as_bytes();
    let b = a.as_bytes().to_vec();
}

fn demo5() {
    println!("\ndemo5");
    let s = String::from("中国aad你好");
    let chararr: Vec<char> = s.chars().collect();
    println!("{:?}", chararr);

    for ch in s.chars() {
        println!("{}", ch);
    }
}
pub fn main() {
    demo1();
    demo2();
    demo3();
    demo4();
    demo5();
}
