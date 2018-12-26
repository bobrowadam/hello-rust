pub mod vector {
    pub fn create_vector() {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        let other_v = vec![1, 2, 3, 4];
        println!("v: {:#?};\nother_v: {:#?}", v, other_v);

        // Will panic on non exists index:
        let first_element: &i32 = &v[0];
        // Will return None on non exists index
        let none_exists = v.get(10);
        // Will return Some on exists index
        let exists = v.get(1);
        println!(
            "first_element: {:?}\nnone_exists: {:?}\nexists: {:?}",
            first_element, none_exists, exists
        );
    }
}
