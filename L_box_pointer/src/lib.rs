
mod lib_ref;

use crate::lib_ref::box_lib::box_lib_runner;
use crate::lib_ref::list_box_lib::list_box_lib_runner;
use crate::lib_ref::list_ref_lib::list_ref_lib_runner;

pub fn runner() {
    println!("lib.rs::runner");
    box_lib_runner();
    list_box_lib_runner();
    list_ref_lib_runner();
}