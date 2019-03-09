use std::fmt;

pub struct MyStruct {
    name: String,
    age: u8,
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}

impl MyStruct {
    pub fn new(name: &String, age: u8) -> MyStruct {
        MyStruct {
            name: name.clone(),
            age,
        }
    }
}
