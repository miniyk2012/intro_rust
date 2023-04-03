mod lifetime;
mod thread;
mod trait_demo;
pub fn main() {
    println!("\nlifetime\n");
    lifetime::example1();
    lifetime::example2();
    lifetime::example3();
    lifetime::example4();

    println!("\nthread\n");
    thread::closure_example();
    thread::example2();
    thread::example3();
    thread::example4();

    println!("\ntrait_demo");
    trait_demo::example1();
    trait_demo::example2();
    trait_demo::example3();
}
