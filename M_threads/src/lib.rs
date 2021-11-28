
mod lib_threads;

use crate::lib_threads::lib_threads_1::lib_threads_1_runner;


pub fn runner() {
    println!("lib runner");
    lib_threads_1_runner();
}
