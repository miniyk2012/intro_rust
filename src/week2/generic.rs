#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn genericDemo1() {
    println!("\ngenericDemo1");
    let both_integer = Point { x: 5, y: 10u64 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!(
        "{:?}, {:?}, {:?}",
        both_integer, both_float, integer_and_float
    );

    let a = Foo::<A> {
        state: PhantomData::<A>,
    };
    let b = a.turn_to_medium();
    println!("{:?}", b);
    let c = b.turn_to_high();
    println!("{:?}", c);
}

use std::marker::PhantomData;

#[derive(Debug)]
struct Foo<T> {
    state: PhantomData<T>,
}

#[derive(Debug)]
struct A;
#[derive(Debug)]
struct B;
#[derive(Debug)]
struct C;

impl Foo<A> {
    fn turn_to_medium(self) -> Foo<B> {
        Foo::<B> {
            state: PhantomData::<B>,
        }
    }
}
impl Foo<B> {
    fn turn_to_low(self) -> Foo<A> {
        Foo { state: PhantomData }
    }
    fn turn_to_high(self) -> Foo<C> {
        Foo { state: PhantomData }
    }
}
impl Foo<C> {
    fn turn_to_medium(self) -> Foo<B> {
        Foo { state: PhantomData }
    }
}

impl Point<u32, u32> {
    fn width(&self) -> u32 {
        return self.x;
    }
}

impl<T, U> Point<T, U> {
    fn height(&self) -> &U {
        return &self.y;
    }
}

pub fn monomorphism() {
    println!("\nmonomorphism");
    let p = Point::<u32, u32> { x: 10, y: 20 };
    println!("width={}", p.width());
    println!("height={}", p.height());

    let p2 = Point::<i32, u32> { x: 10, y: 30 };
    // println!("width={}", p2.width());
    println!("height={}", p2.height());
}
