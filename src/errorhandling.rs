pub(crate) mod errorHandling{
    pub fn errorHandling(){
        println!("Starting errorHandling..");

        let gift = "Snake";
        handleError(gift);

        let mut list = Vec::new();
        list.push(1);
        list.push(2);

        let result = checkList(list);
        if result.is_ok(){
            println!("got value: {}", result.unwrap());
        }
        else{
            println!("got error: {}", result.unwrap_err())
        }
    }

    fn checkList(list: Vec<i32>) -> Result<i32, &'static str>{
        println!("Checking list..");
        println!("First element: {}", list[0]);
        let x = list.get(0);
        match list.get(0){
            Some(x) => Ok(*x),
            None => Err("Error: value not found")
        }
    }

    fn handleError(gift : &str){
        if gift == "Snake"{
            //panic!("Ohh we got the Snake");
        }
    }

    #[derive(Clone, Copy)]
    struct PhoneNumber{
        area_code: Option<u8>,
        number: u32,
    }
}