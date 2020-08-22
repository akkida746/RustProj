#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str{
    "Hello World!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    println!("{}",name);
    println!("Hello Akash");
    let result = format!("Hello {}", name);
    return result;
}

fn main() {
    println!("Inside application");
    // Add routes here
    rocket::ignite().mount("/", routes![index,hello]).launch();
}
