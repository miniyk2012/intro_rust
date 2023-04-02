use std::future::Future;
use tokio::runtime::Runtime;
use tokio::task;
use tokio::time;

// `foo()` returns a type that implements `Future<Output = u8>`.
// `foo().await` will result in a value of type `u8`.
async fn foo() -> u8 {
    5
}

// Future表示一个未来的值
fn bar() -> impl Future<Output = u8> {
    // This `async` block results in a type that implements
    // `Future<Output = u8>`.
    async {
        let x: u8 = foo().await;
        println!("{}", x);
        x + 5
    }
}

async fn bar2() -> u8 {
    let x: u8 = foo().await;
    x + 5
}
pub fn example1() {
    println!("\nexample1");
    // let a = foo();
    // let a = bar();
    // let b = bar2();
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let a = bar().await;
        println!("{}", a);
    });
}

pub fn example2() {
    println!("\nexample2");
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        println!("in tokio task1");
        let join_handle = task::spawn(async {
            println!("in tokio task2");
        });
        bar().await;
        join_handle.await;
    });
}

async fn task_that_takes_a_second() {
    println!("hello");
    time::sleep(time::Duration::from_secs(1)).await
}

#[tokio::main]
pub async fn interval_example() {
    println!("\ninterval_example");
    let mut interval = time::interval(time::Duration::from_secs(2));
    for _ in 0..5 {
        interval.tick().await;
        task_that_takes_a_second().await;
    }
}
