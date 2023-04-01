use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

const FIRST_FILE: &str = "first.txt";
const SECOND_FILE: &str = "second.txt";

pub fn writefile() -> Result<(), io::Error> {
    let mut path_buf = PathBuf::new();
    path_buf.push("src/week2");
    path_buf.push(FIRST_FILE);

    let mut f = File::create(path_buf)?;

    let numbers: Vec<i32> = (1..100).collect();
    println!("{}", numbers.len());
    for number in &numbers {
        if number % 3 == 0 {
            f.write(format!("{}\n", number).as_bytes())?;
        } else {
            f.write(format!("{}, ", number).to_string().as_bytes())?;
        }
    }
    f.flush()?;
    Ok(())
}

fn reverseNumber() -> Result<(), io::Error> {
    let mut read_path = PathBuf::new();
    read_path.push("src/week2");
    read_path.push(FIRST_FILE);

    let f = File::open(read_path)?;
    let mut totalNumbers: Vec<String> = Vec::new();
    for line in io::BufReader::new(f).lines() {
        let strline = line.unwrap();
        let numbers: Vec<&str> = strline.split(",").collect();
        for &number in &numbers {
            totalNumbers.push(number.trim().to_owned());
        }
    }

    let mut write_path = PathBuf::new();
    write_path.push("src/week2");
    write_path.push(SECOND_FILE);
    let mut writeFile = File::create(write_path)?;
    let mut count = 0;
    for v in totalNumbers.iter().rev() {
        count += 1;
        if count % 3 == 0 {
            writeFile.write(format!("{}\n", v).as_bytes())?;
        } else {
            writeFile.write(format!("{}, ", v).to_string().as_bytes())?;
        }
    }
    Ok(())
}

pub fn writeReverNumberToFile() -> Result<(), io::Error> {
    writefile()?;
    reverseNumber()?;
    Ok(())
}
