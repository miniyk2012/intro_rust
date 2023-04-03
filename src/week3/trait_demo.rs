use std::fmt::Display;

trait Animal {
    fn eat(&self) {
        println!("Animal eat");
    }
}

trait MeatAnimal: Animal {
    fn eat_meat(&self);
}

trait VegetableAnimal {
    fn eat_vegetable(&self) {
        println!("VegetableAnimal eat");
    }
}

struct Rabbit;

impl Animal for Rabbit {
    fn eat(&self) {
        println!("Rabbit eat");
    }
}

impl VegetableAnimal for Rabbit {
    fn eat_vegetable(&self) {
        println!("Rabbit eat vegetable");
    }
}

struct Goat;

impl Animal for Goat {}

impl VegetableAnimal for Goat {}

struct Tiger;

impl Animal for Tiger {}

impl MeatAnimal for Tiger {
    fn eat_meat(&self) {
        println!("tiger eat meat");
    }
}

// 参数使用引用是因为默认结构体不会实现Copy
fn foo<T: Animal>(_a: &T) {}

fn foo_veg<T: VegetableAnimal>(_a: &T) {}

fn foo_meat<T: MeatAnimal>(_a: &T) {}

pub fn example1() {
    println!("\nexample1");
    let r = Rabbit;
    r.eat();  // Rabbit eat
    r.eat_vegetable();  // Rabbit eat vegetable

    Animal::eat(&r);  // Rabbit eat
    <Rabbit as VegetableAnimal>::eat_vegetable(&r);  // Rabbit eat vegetable

    let t = Tiger {};
    t.eat();  // Animal eat
    Animal::eat(&t);  // Animal eat
    <Tiger as Animal>::eat(&t);  // Animal eat
    <Tiger as MeatAnimal>::eat_meat(&t);  // tiger eat meat

    let g = Goat;
    foo(&r);
    foo(&t);
    foo_veg(&g);
    foo_meat(&t);
}

// 关联类型
trait StreamIterator {
    type Item: Display;   // 在trait上具有某个类型, 比如String
}

struct Foo<T>
    where T: StreamIterator<Item=String> {
    x: T,
}

#[derive(Debug)]
struct A;

impl StreamIterator for A {
    type Item = String;
}


pub fn example2() {
    println!("\nexample2");
    let f = Foo {
        x: A,
    };
    let s = <A as StreamIterator>::Item::from("abc");
    println!("{:?}", f.x);
}

trait Add {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn use_add<K, T: Add<Rhs=K, Output=K>>(p1: T, p2: K) -> K {
    return p1.add(p2);
}

pub fn example3() {
    println!("\nexample3");
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let px = Point { x: 1, y: 1 };
    let py = Point { x: 1, y: 1 };
    let pz = use_add(px, py);
    assert_eq!(pz.x, 2);
    assert_eq!(pz.y, 2);
    println!("{:?}", pz);
}