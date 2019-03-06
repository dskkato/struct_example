mod mystruct;
use mystruct::MyStruct;

fn main() {
    let name = "hoge";
    let age = 8;
    let _m = MyStruct::new(name, age);
    println!("Hello, world!");
}
