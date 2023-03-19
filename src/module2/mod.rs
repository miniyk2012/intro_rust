use std::boxed::Box;

fn demo21() {
    println!("\ndemo21 堆,栈");
    // 固定大小在栈上，可变大小在堆上(String)
    // struct也存在栈上
    let s = String::from("aaa"); // 局部变量s在栈上, 指向的值在堆里, "aaa"字面值在静态数据区里
    let m = 10; // m放在栈上
    let a = Box::new(m); // a指向的值分在在堆上
    println!("{s}, {m}, {a}")
}

fn demo22() {
    println!("\ndemo22 mut, let");
    let n = 20;
    println!("{n}");
    let mut n = 'b';
    println!("{n}");
    n = 'c';
    println!("{n}");

    let mut x = 5;
    println!("{x}");
    x = 6;
    println!("{x}");
}

fn demo23() {
    println!("\ndemo23");
    let a = 10u32;
    let b = a;
    println!("{a}");
    println!("{b}");

    let s1 = String::from("I am a spuerman");
    let s2 = s1.clone(); // 深拷贝
    println!("{s1}");
    println!("{s2}");
}

fn demo24() {
    let s1 = String::from("I am a superman");
    {
        let s2 = s1;
        println!("{s2}");
    }
    // 所有权转移后不会归还
    // println!("{s1}"); // value borrowed here after move
}

fn foo(s: String) -> String {
    // s会获取传入资源的所有权
    println!("in foo: {s}");
    s
}

#[derive(Debug)]
struct User {
    name: String,
}

fn demo25() {
    println!("\ndemo25");
    let s1 = String::from("im a superman");
    let s1 = foo(s1);
    println!("{s1}");

    let s2 = &s1; // 引用, 是一种借用, 只读
    let s3 = s2; // 不可变引用是复制操作(引用是usize, 固定大小)
    println!("{s3}");
    println!("{s2}");
    println!("{s1}");
    // s2.push_str("abc");  // 借用不能改变资源
    let mut u = User {
        name: String::from("yangkai"),
    };
    u.name = String::from("guzhelun");
    println!("{:?}", u);

    u = User {
        name: String::from("xieshen"),
    };
    println!("{:?}", u);
    let u2 = &u;
    // u2.name = String::from("nihao"); // u2` is a `&` reference, so the data it refers to cannot be written
    println!("{:?}", u2);
}

fn demo26() {
    println!("\ndemo26");
    let a = String::from("hello");
    let b = &a;
    let c = &&&a;
    let d = c;
    // let e = a;
    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
}

fn bar(s: &String) {
    // 借用
    println!("in bar: {s}")
}

fn demo27() {
    println!("\ndemo27");
    let s1 = String::from("hello");
    let s2 = &s1; // 引用, 即借用, 借用时所有权不会转移
    bar(s2);
    println!("{s1}");

    let mut a = 10u32;
    let b = &mut a; // 可变引用
    *b = 20;
    println!("{b}");
    println!("{a}"); // println!会对变量做不可变引用操作, 等于是let c = &a;
}

fn demo28() {
    println!("\ndemo28");
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    println!("{b}"); // b的作用域结束

    // 类似语言层面的锁
    let d = &mut a; // d的作用域开始
    *d = 30;
    println!("{d}"); // d的作用域结束
}

fn demo29() {
    println!("\ndemo29");
    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;

    *r2 = 20;
    println!("{r2}");

    a = 30;
    println!("{a}");
}

fn demo30() {
    let mut a1 = 10u32;
    let mut a2 = 12u32;
    let mut b = &mut a1;

    b = &mut a2;
    let mut c = &a1;
    c = &mut b;
}

fn demo31() {
    println!("\ndemo31 多级引用");
    let mut a1 = String::from("abc");
    let mut b = &mut a1;

    *b = String::from("agc");

    let c = &mut b;
    **c = String::from("def"); // 其实还是同过b对a1写, 所以不重叠
    println!("{c}");

    println!("{b}");
}

fn foo2(s: &mut String) {
    // 借用
    s.push_str(" You are batman.");
}

fn demo32() {
    println!("\ndemo32 多级引用");
    let mut s = String::from("hello,");
    foo2(&mut s);
    println!("{s}");
}

fn demo33() {
    println!("\ndemo33 多级引用");
    let mut a = 10u32;
    let _x = a;
    a = 100;

    let b = &mut a;
    *b = 20;

    let c = &mut a;
    *c = 30;
}

fn demo34() {
    println!("\ndemo34 多级引用");
    // 关键是资源a1只有b在引用
    let mut a1 = 10u32;

    let mut b = &mut a1;
    let mut c: &mut &mut u32 = &mut b;

    let d: &mut &mut &mut u32 = &mut c;
    // println!("{c}"); // 资源c同时也d被借用着

    // *b = 20; // 资源b同时被c借用着

    ***d = 100; // d虽然和c作用域重叠, 但d借用的是资源c, c借用的是资源b, 因此不冲突
    println!("{c}");
    **c = 30;
    *b = 20;
}

fn fooint(s: &mut i32) {
    *s = 123;
}
fn demo35() {
    println!("\ndemo35");
    let mut s = 100;
    println!("{s}");
    fooint(&mut s);
    println!("{s}");
}

fn demo36() {
    println!("\ndemo36");
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    println!("{b}");
    a = 30;
    println!("{a}");

    let c = &a;
    let d = &a;
    println!("{a}");
    println!("{c}");
}

fn demo37() {
    println!("\ndemo37");
    let a = String::from("hello");
    let mut b = a;
    b = String::from("helwo");
    let r1 = &mut b;
    let r2 = r1;
    // println!("{r1}"); // 会报错, 可变引用只能被move
    println!("{r2}");

    let r3 = &b;
    let r4 = r3;
    println!("{r3}"); // 可变借用会被复制, 因此仍然可以用
    println!("{r4}");
}

fn demo38() {
    println!("\ndemo38");
    let mut a = 100;
    let b = &a;
    println!("{a}");
    println!("{b}");
    a = 20;
    println!("{a}");
}

fn demo39() {
    println!("\ndemo39");
    let mut a1 = 10u32;
    let mut a2 = 15u32;
    let mut b = &mut a1;
    b = &mut a2;
    let mut c = &a1;
    c = &a2;
}

fn interesting() {
    println!("\ninteresting");
    let mut a = 10u32;
    println!("{a}");
    a = 15;

    let mut b = &mut a;
    let mut c = &mut b;
    let d = &mut c;
    println!("{d}");
    // **c = 101; // 这行会报错, 因为上面d已经把c借走了, 而且还未归还(未出作用域)

    // println!("{b}"); // 这会报错, 是因为上面c把b可变借用了, 而d又把c可变借用了,下面d还在用, 因此b不能用
    println!("{d}");
}
fn foo3(a: u32) {}
fn foo4(a: &u32) {
    println!("{a}");
}

fn interesting2() {
    println!("\ninterestin2g");
    let mut a = 10u32;
    println!("{a}");
    a = 15;
    let b = &mut a;
    *b = 100;
    // foo3(a); // 会报错, 因为等于是let c = a;  use of borrowed `a`, a暂时借出所有权
    // foo4(&a); // 报错, &a与b冲突, 同下
    // println!("{a}"); // 这里会报错, 因为在b作用域内, 不能有对a的不可变引用

    let mut c = &b;
    let d = &mut c;
    // *b = 100; // 这里会报错, 因为c借用b了, b不可以再赋值
    println!("{b}"); // 这不会报错, 是因为上面c对b是不可变借用
    println!("{d}");
}

fn demo40() {
    println!("\ndemo40");
    let mut a = 10u32;
    let b = &mut a;
    // let c = &a;
    // println!("{a}");
    println!("{b}");
    *b = 20;
    let c = &a;
    println!("{c}");
}

fn demo41() {
    println!("\ndemo41");
    let mut a = 10u32;
    let b = &a;
    // a = 20; // 会报错, assignment to borrowed `a` occurs here
    println!("{b}");
    a = 30;
    println!("{a}");
}

fn demo42() {
    println!("\ndemo41");
    let mut a1 = 10u32;
    let b = &mut a1;
    let mut c = &b;
    let d = &mut c;
    // ***d = 30;  // 只有全是多级可变引用的情况下，才能修改到目标资源的值
    println!("{d}");
}

fn foo5(a: u32) {}
fn foo6(a: String) {}

fn demo43() {
    println!("\ndemo43");
    let a = 10u32;
    foo5(a); // 复制, 所以没关系
    println!("{a}");

    let b = String::from("value");
    foo6(b);
    // println!("{b}"); // foo6(b)已经把b的所有权转移了
}

fn foo7(mut s: String) {
    s.push_str(" You are batman.");
    s = String::from("I am a superman.");
}

fn foo8(r2: &mut String) {
    r2.push_str(" You are batman.");
}
fn demo44() {
    let mut s1 = String::from("I am a superman.");
    println!("{s1}");
    foo7(s1);
    // println!("{s1}"); // 由于转移了所有权, 导致不能用了

    let mut s2 = String::from("I am a superman.");
    let r2 = &mut s2;
    foo8(r2);
    println!("{r2}");
    println!("{s2}");
}

fn demo45() {
    let unicode_codepoint = String::from("\u{003e}");
    println!("{unicode_codepoint}");
}
pub fn main() {
    println!("\n ---------module2----------");
    demo21();
    demo22();
    demo23();
    // demo24();
    demo25();
    demo26();
    demo27();
    demo28();
    demo29();
    demo30();
    demo31();
    demo32();
    demo33();
    demo34();
    demo35();
    demo36();
    demo37();
    demo38();
    demo39();
    interesting();
    demo40();
    interesting2();
    demo41();
    demo42();
    demo43();
    demo44();
    demo45();
}
