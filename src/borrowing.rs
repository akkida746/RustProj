pub(crate) mod borrowing{
    pub fn borrowing(){
        print!("Inside borrowing()");

        /*Box is used for allocating memory in Heap*/
        let boxed_i32 = Box::new(5_i32);
        let stacked_i32 = 6_i32;
    }

    /*Borrowing i32, but ownership is not taken*/
    fn borrow_i32(borrowed_i32: &i32){
        print!("This int is: {}", borrowed_i32);
    }

    /*take ownership and destroys it*/
    fn eat_box_i32(boxed_i32: Box<i32>){
        println!("Destroying box that contains {}", boxed_i32);
    }

}

pub(crate) fn getModuleName() -> String{
    "borrowing".to_string()
}