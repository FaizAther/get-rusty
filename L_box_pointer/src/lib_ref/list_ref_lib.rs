//  Reference Counting

use std::rc::Rc;

use crate::lib_ref::list_box_lib::MyList::{Cons, Nil};

fn ref_run_1() {
    println!("ref_run_1");
}

pub fn list_ref_lib_runner() {
    ref_run_1();
}
