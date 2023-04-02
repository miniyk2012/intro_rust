mod lifetime;
mod thread;
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
}
