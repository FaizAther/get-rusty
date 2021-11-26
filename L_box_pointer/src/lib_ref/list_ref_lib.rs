//  Reference Counting

use crate::lib_ref::list_box_lib::MyList::{Cons, Nil};

fn ref_run_1() {
    println!("ref_run_1");

    let l_box = Cons::<u8>(5, Box::new(Nil));
    println!("\t{:?}", l_box);
}

use std::rc::Rc;

#[derive(Debug)]
pub enum MyList<T> {
    Cons(T, Rc<MyList<T>>),
    Nil
}

fn ref_run_2() {
    println!("ref_run_2");

    let a = Rc::new(MyList::Cons::<u8>(5, Rc::new(MyList::Nil)));
    println!("\t{:?}", a);
    let b = Rc::new(MyList::Cons::<u8>(6, a));
    println!("\t{:?}", b);
    let c = Rc::new(MyList::Cons::<u8>(7, b));
    println!("\t{:?}", c);

//  println!("\t{:?}\n\t{:?}\n\t{:?}", a, b, c);
}

fn ref_run_3() {
    println!("ref_run_3");

    let a = Rc::new(MyList::Cons::<u8>(5, Rc::new(MyList::Nil)));
    println!("\t{:?}", a);
    let b = Rc::new(MyList::Cons::<u8>(6, a.clone()));
    println!("\t{:?}", b);
    let c = Rc::new(MyList::Cons::<u8>(7, Rc::clone(&b)));
    println!("\t{:?}", c);

    println!("\t{:?}\n\t{:?}\n\t{:?}", a, b, c);
}

fn ref_run_4() {
    println!("ref_run_4");
    let a = Rc::new(MyList::Cons(5, Rc::new(MyList::Nil)));
    println!("\ta={:?} a.ref_count=`{}`", a, Rc::strong_count(&a));

    let b = Rc::new(MyList::Cons(6, a.clone()));
    println!("\tb={:?} a.ref_count=`{}`, b.ref_count=`{}`",
             a, Rc::strong_count(&a), Rc::strong_count(&b));
    {
        let c = Rc::new(MyList::Cons(7, a.clone()));
        println!(
            "\tb={:?} a.ref_count=`{}`, b.ref_count=`{}`, c.ref_count=`{}`",
            a, Rc::strong_count(&a), Rc::strong_count(&b), Rc::strong_count(&c)
        );
        println!("\t{:?}\n\t{:?}\n\t{:?}", a, b, c);
    }
    println!(
            "\tb={:?} a.ref_count=`{}`, b.ref_count=`{}`",
            a, Rc::strong_count(&a), Rc::strong_count(&b)
        );
}

pub fn list_ref_lib_runner() {
    ref_run_1();
    ref_run_2();
    ref_run_3();
    ref_run_4();
}
