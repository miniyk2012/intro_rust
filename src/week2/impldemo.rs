struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn numbers(row: u32, col: u32) -> u32 {
        row * col
    }
}

pub fn demo1() {
    println!("\ndemo1");
    println!("numbers={}", Rectangle::numbers(4, 5));
}
