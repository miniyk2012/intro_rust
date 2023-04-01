use std::io;
use std::{fs::File, io::Read};

fn read_content_from_file1() -> Result<String, io::Error> {
    let file1 = File::open("src/week2/hello.txt");
    let mut greeting_file = match file1 {
        Ok(file) => file,
        Err(e) => return Err(e), // return可以直接退出read_content_from_file函数, 并返回Err(io::Error)
    };
    println!("owo");
    let mut contents = String::new();
    match greeting_file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn read_content_from_file2() -> Result<String, io::Error> {
    let mut file = File::open("src/week2/hello.txt")?; // 如果失败会return Err(io::Error), 逻辑和read_content_from_file1一致
    let mut content = String::new();
    // let a = "100".parse::<u32>()?;
    file.read_to_string(&mut content)?; // 如果失败会return Err(io::Error)
    Ok(content)
}

pub fn openFile() {
    println!("\nopenFile");
    let s = read_content_from_file1();
    match s {
        Ok(x) => println!("read content: {:?}", x),
        Err(e) => println!("fail to read: {}", e),
    }

    let s2 = read_content_from_file2();
    match s2 {
        Ok(x) => println!("read content: {:?}", x),
        Err(e) => println!("fail to read: {}", e),
    }

    let text = "001
    002
    ";
    println!("{:?}", last_char("")); // None
    println!("{:?}", last_char(text)); // Some('1')
}

fn last_char(text: &str) -> Option<char> {
    text.lines()
        .next()? // 如果None, 直接返回None
        .chars()
        .last()
}
