#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod Types;
use crate::Types::Types::User;

#[get("/")]
fn index() -> &'static str{
    "Hello World!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    println!("{}",name);
    println!("Hello Akash");

    let mut user = User{
        name: "Akash".parse().unwrap(),
        age: 32,
    };

    let result = format!("Hello {}", user.name);

    return result;
}

fn main() {
    println!("Inside application");
    // Add routes here
    rocket::ignite().mount("/", routes![index,hello]).launch();
}
