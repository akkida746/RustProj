pub fn public_function(){
    println!("Inside rary's public_function()");
}

fn private_function(){
    println!("Inside rary's private_function()")
}
pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
