use std::fmt::Debug; //Trait to bound with

pub(crate) mod bounds{
    use std::fmt::Debug;

    pub fn startBounds(){
        println!("Starting bounds..");
    }

    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    // A generic function which prints using the 'Debug' trait
    fn print<T>(t: T) where T: Debug{
        println!("`print`: t is {:?}", t);
    }
}