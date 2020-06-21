pub(crate) mod coercion{
    pub fn coercion(){
        println!("Starting Coercion..");

        let first = 2;
        {
            let second = 3;
            println!("After multiply: {}", multiply(&first, &second));
            println!("{} is the first", choose_first(&first, &second));
        }
    }

    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32{
        first * second
    }

    /*Here we have defined a lifetime 'a which is as long as lifetime of 'b*/
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32{
        first
    }

}