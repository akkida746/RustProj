mod borrowing;
mod module;

use crate::module::module::printName;

fn main() {
    println!("Hello, world!");

    let name = Employee::Name("Akash".to_string());
    print!("{:?}", name);
    printName();

    let data = Data{name:"Akash".to_string(), age:32,optional_num: None};
    print!("{}, {}", data.name, data.age);
    print!("{}", data.getAge());
    let data1 = Data::new();
    print!("{}", data1.getAge());
}

struct Data{
    name: String,
    age: i32,
    optional_num: Option<i32>
}

//Adding methods in struct
impl Data{
    fn new() -> Self{
        Data{
            name: "Deep".to_string(),
            age: 31,
            optional_num: None
        }
    }
    fn getAge(&self) -> i32{
        self.age
    }
}

#[derive(Debug)]
enum Employee {
    Name(String),
    Id(i32),
    Profile(String),
}

