#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str{
    "Hello World!"
}

fn main() {
    println!("Inside application");

    rocket::ignite().mount("/", routes![index]).launch();
}
