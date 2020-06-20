mod module;

use crate::module::module::printName;

fn main() {
    println!("Hello, world!");

    let name = Employee::Name("Akash".to_string());
    print!("{:?}", name);

    printName();
}

#[derive(Debug)]
enum Employee {
    Name(String),
    Id(i32),
    Profile(String),
}

