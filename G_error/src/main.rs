
// Concept 1: Spagetti

fn my_err_1() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(2); //pass 22 here for err
}

fn c(c: i32) {
    if c == 22 {
        panic!("Died");
    }
}

// Concept 2: Grecefully

// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

use std::io::ErrorKind;
use std::io::prelude::*;
use std::fs::File;

fn my_err_2(path: &str) {
    let f = File::open(path);
    let file: File = match f {
        Ok(file) => file,
        Err(r) => {
            match r.kind() {
                ErrorKind::NotFound =>
                    match File::create("bad.txt") {
                        Ok(ef) => {println!("{:?}", ef); ef},
                        Err(er) => panic!("create: {:?}", er)
                },
                oer => panic!("other error: {:?}", oer)
            }
        }
    };

    for byte in file.bytes() {
        let byte = match byte {
            Ok(byte) => byte,
            Err(r) => panic!("bad {:?}", r)
        };
        println!("byte: {:?}", byte);
    }

}

fn main() {
    my_err_1();
    my_err_2("./src/main.rs");
}
