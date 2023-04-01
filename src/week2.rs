mod generic;
mod hashmap;
mod homework;
mod impldemo;
mod optiondemo;
mod result;
pub fn main() {
    hashmap::insertDemo();
    hashmap::entryDemo();
    hashmap::getDemo();
    hashmap::mutGetDemo();
    hashmap::iterDemo();
    hashmap::iterMutDemo();
    println!("\nimpldemo");
    impldemo::demo1();
    println!("\noptiondemo");
    optiondemo::optionDemo1();
    optiondemo::enumDemo();
    optiondemo::matchDemo1();
    optiondemo::whileLet();
    optiondemo::matchStruct();
    optiondemo::matchFun();
    optiondemo::consumeSelf();

    println!("\n\ngenericdemo");
    generic::genericDemo1();
    generic::monomorphism();

    println!("\nresult");
    result::openFile();

    println!("\nhomework");
    homework::writeReverNumberToFile();
}
