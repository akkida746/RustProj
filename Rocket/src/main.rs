#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod Types;
use crate::Types::Types::User;
use rocket_contrib::json::Json;
use serde_json::Value;
#[macro_use]
extern crate serde_json;

// Reading json from request body and sending json as response
#[post("/create", format = "application/json", data = "<new_book>")]
fn new(new_book: Json<User>) -> Json<Value> {
    println!("Inside new method");
    return Json(json!({
        "status": "Akash",
    }));
}

#[get("/")]
fn index() -> &'static str{
    "Hello World!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    println!("{}",name);
    println!("Hello Akash");

    let mut user = User{
        name: "Akash".parse().unwrap()
    };

    let result = format!("Hello {}", user.name);

    return result;
}

fn main() {
    println!("Inside application");
    // Add routes here
    rocket::ignite().mount("/", routes![index,hello,new]).launch();
}
