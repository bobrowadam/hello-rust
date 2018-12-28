pub mod errors {
    use std::fs::File;
    use std::io::ErrorKind;

    // pub fn make_errors() {
    //     let v = vec![1, 2, 3];
    //     v[99];
    // }

    pub fn file_error() -> std::fs::File {
        match File::open("hello.txt") {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                },
                other_error => panic!("There was a problem opening the file: {:?}", other_error),
            },
        }
    }
}
