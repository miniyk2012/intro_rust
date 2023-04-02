use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    let ori = Point { x: 0.0, y: 0.0 };

    ori
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn example1() {
    println!("\nexample1");
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let mut boxed_point: Box<Point> = Box::new(origin());
    println!("{:?}", boxed_point);
    println!("x={}", boxed_point.x);

    *boxed_point = Point { x: 1.0, y: 0.0 };
    println!("{:?}", boxed_point);
    println!("x={}", boxed_point.x);
}
#[derive(Debug)]
struct A;
pub fn example2() {
    println!("\nexample2");
    let a = A {};
    let b = Box::new(a);
    // println!("{:?}", a); // ^ value borrowed here after move
    println!("{:?}", b); // ^ value borrowed here after move
}

use std::time::Duration;

use std::{thread, thread::sleep};

use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct User {
    name: String,
}

pub fn example3() {
    println!("\nexample3");
    let user_original = Arc::new(Mutex::new(User {
        name: String::from("drogus"),
    }));

    let user = user_original.clone();

    let t1 = thread::spawn(move || {
        let mut locked_user = user.lock().unwrap();

        locked_user.name = String::from("piotr");

        // after locked_user goes out of scope, mutex will be unlocked again,

        // but you can also explicitly unlock it with:

        // drop(locked_user);
    });

    let user = user_original.clone();

    let t2 = thread::spawn(move || {
        sleep(Duration::from_millis(10));

        // it will print: Hello piotr

        println!("Hello {}", user.lock().unwrap().name);
    });

    t1.join().unwrap();

    t2.join().unwrap();
}
