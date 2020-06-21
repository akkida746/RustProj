pub(crate) mod lifetime{
    pub fn start(){
        println!("Starting lifetime..");
        let x = "Akash";
        let y = "Deep";
        let z= largest(x);
        println!("{:?}", z);
    }

    /*static lifetime remains untill the project is running*/
    fn longest(x: &str, y: &str) -> &'static str{
        "abc"
    }

    /*Lifetime is  required when a function is returning a reference, compile needs to know till when this reference needs to be kept*/
    /*Here we have give the returned value the lifetime of input reference x, i.e 'a*/
    fn largest<'a>(x: &'a str) -> &'a str{
        x
    }
}