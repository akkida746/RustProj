pub(crate) mod collections{

    const NAME: &str = "Akash Deep";

    pub fn startCollections(){
        println!("Starting Collections");

        let mut list = Vec::new();
        list.push(1);
        changeValues(list);

        let x:Option<&str> = Some("Hello World");
        assert_eq!(x.is_some(), true);
        println!("Running by: {}", NAME);
    }

    fn changeValues(mut list: Vec<i32>){
        println!("Before value change: {:?}", list.get(0).unwrap());
        list[0] = 3;
        println!("After value change: {:?}", list.get(0).unwrap());
    }
}