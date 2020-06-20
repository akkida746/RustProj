pub(crate) mod module{
    pub fn printName(){
        println!("Inside PrintName()");

        let mut i=0;
        while i<10{
            println!("{}", i);
            i = i + 1;
        }

        let x = 2;
        check(x);
        print!("After check :{}", x);
    }

    fn check(mut x: i32){
        x = x + 1;
        print!("x: {}", x);
    }
}
