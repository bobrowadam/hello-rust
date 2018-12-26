pub mod strings_stuff {
    pub fn print_strings() {
        let my_string = "some string";
        let mut other_string = String::from(my_string);
        other_string.push_str(" 2");
        println!(
            "first string:{},\nsecond string:{}",
            my_string, other_string
        );
        // let hello = String::from("hello");
        let hello = String::from("hello");
        let world = String::from("world");
        let s3 = hello + " " + &world;
        println!("Concatenated string: {}", s3);
    }
}
