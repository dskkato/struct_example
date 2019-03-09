mod mystruct;
use mystruct::MyStruct;

fn main() {
    let name = String::from("hoge");
    let age = 8;
    let m = MyStruct::new(&name, age);
    println!("Hello, {}", m);
}
