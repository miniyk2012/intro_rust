struct Rabbit;

trait Animal {
    fn eat(&self);
}

trait MeatAnimal: Animal {
    fn eat_vegetable(&self);
}

trait VegetableAnimal {
    fn eat_meat(&self);
}

struct Rabbit;
impl Animal for Rabbit {
    fn eat(&self) {}
}

impl VegetableAnimal for Rabbit {
    fn eat_vegetable(&self) {}
}

struct Goat;

impl Animal for Goat {
    fn eat(&self) {}
}

impl VegetableAnimal for Goat {
    fn eat_meat(&self) {}
}

pub fn example1() {
    println!("\nexample1");
}

trait StreamIterator {
    type Item;
}
