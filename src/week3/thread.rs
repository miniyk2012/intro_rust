pub fn closure_example() {
    println!("\nclosure_example");
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure("d".to_string());
    println!("s={}, n={}", s, n);
}
#[derive(Debug)]
struct B;
use std::thread;
pub fn example2() {
    println!("\nexample2");
    let data = vec![1, 2, 3];
    // let a = 10;
    let _b = B;
    let handle = thread::spawn(move || {
        println!("data={:?}", data);
        // println!("{:?}", b);
        // println!("a={:?}", a);
        for item in data {
            println!("{}", item);
        }
    });

    // `data` has been moved to the new thread, so we can't access it here
    // println!("{:?}", data);

    handle.join().unwrap();
}

pub fn example3() {
    println!("\nexample3");
    let v = vec![1, 2, 3];

    std::thread::scope(|s| {
        s.spawn(|| {
            println!("Here's a vector: {:?}", v);
            // v.push(4);
        });
        s.spawn(|| {
            println!("Here's a vector: {:?}", v);
            // v.push(5);
        });
        println!("main v={:?}", v);
    });
    println!("after={:?}", v);
}

pub fn example4() {
    println!("\nexample4");
    let mut v = vec![1, 2, 3];

    std::thread::scope(|s| {
        println!("before main v={:?}", v);
        s.spawn(|| {
            println!("Here's a vector: {:?}", v);
            v.push(4);
        });
    });
    println!("after main={:?}", v);
}
