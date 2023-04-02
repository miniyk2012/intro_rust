mod asyncstudy;
mod echo;
mod pointer;
pub fn main() {
    println!("\nweek4\npointer\n");
    pointer::example1();
    pointer::example2();
    pointer::example3();

    println!("\nasyncstudy\n");
    asyncstudy::example1();
    asyncstudy::example2();
    asyncstudy::interval_example();

    // echo::run();
}
