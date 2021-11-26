// Smart Pointers

use std::fmt::Display;

mod lib;

#[derive(Debug)]
enum MyThing<T, U>
    where T: PartialOrd,
          U: Display
{
    NumericData(T),
    TextData(U)
}

#[derive(Debug)]
enum MyList<T> {
    Cons(T, Box<MyList<T>>),
    Nil
}

fn box_run() {
    let b: Box<u32> = Box::new(5);
    println!("{}", b);

    let nil: Box<MyList<i32>> = Box::new(MyList::<i32>::Nil);
    let list = Box::new(MyList::<i32>::Cons(44, nil));
    let list1 = Box::new(MyList::<i32>::Cons(55, list));
    println!("{:?}", list1);

    let nill = Box::new(MyList::<MyThing<i32, &str>>::Nil);
    let thing1 = MyThing::TextData("thing1");
    let listt = Box::new(
        MyList::<MyThing<i32, &str>>::Cons(thing1, nill)
    );
    let thing2 = MyThing::NumericData(44);
    let listtt = Box::new(
        MyList::<MyThing<i32, &str>>::Cons(thing2, listt)
    );
    println!("{:?}", listtt);
}

fn main() {
    box_run();
    crate::lib::deref();
}
