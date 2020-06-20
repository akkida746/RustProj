mod borrowing;
mod module;

use crate::module::module::printName;
use crate::house::printHouse;
use crate::deeply::nested::function as function;

fn main() {
    println!("Hello, world!");

    let name = Employee::Name("Akash".to_string());
    print!("{:?}", name);
    printName();

    let data = Data{name:"Akash".to_string(), age:32,optional_num: None};
    println!("{}, {}", data.name, data.age);
    println!("{}", data.getAge());
    let data1 = Data::new();
    println!("{}", data1.getAge());
    println!("Implementing Calculator");
    println!("{}",Calculator::add(1,2));
    println!("Implementing trait");
    data.output_rev();
    printHouse();
    function();
    house::printHouse();

    my::callSelfFoo();
    my::callParentFoo();
    println!("{}", borrowing::getModuleName().to_string());

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    println!("Printing values");
    println!("{}", boxed_i32);
    println!("{}", stacked_i32);

    println!("Mutability");
    let mut x:i32 = 1;
    mutation(x);
    println!("x = {}", x);
    println!("Changing with reference");
    reference(&mut x);
    println!("x after reference {}", x);

}

/*Accessing values using reference is also called Borrowing*/
fn reference(x: &mut i32){
    *x = *x + 1;
}

fn mutation(mut x: i32){
    x = x + 1;
    println!("x after mutation {}", x);
}

fn eat_box_i32(boxed_i32: Box<i32>){
    println!("Destroying box that contains {}", boxed_i32);
}

/*Taking the reference but not taking ownership*/
fn borrow_i32(borrowed_i32: &i32){
    println!("This int is: {}", borrowed_i32);
    println!("-> {}", *borrowed_i32);
}

mod deeply{
    pub mod nested{
        pub fn function(){
            println!("Inside function()")
        }
    }
}

fn foo(){
    println!("Inside parent foo()");
}

mod my{
    fn foo(){
        println!("Inside self foo()")
    }
    pub(crate) fn callSelfFoo(){
        self::foo();
    }
    pub(crate) fn callParentFoo(){
        super::foo();
    }
}

mod house{
    pub(crate) fn printHouse(){
        println!("Inside house()")
    }
}

trait Transform{
    fn rev(&self) -> String;
    fn output_rev(&self){
        println!("{}", self.rev());
    }
}

impl Transform for Data{
    fn rev(&self) -> String {
        self.name.chars().rev().collect::<String>()
    }
}

struct Calculator;

impl Calculator{
    fn add(a: i32, b: i32) -> i32{
        a+b
    }
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

