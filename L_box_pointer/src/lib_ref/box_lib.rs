//  Deref Lib

fn deref_1() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_2() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(item: T) -> MyBox<T> {
        return MyBox::<T>(item);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

//  fn deref(&self) -> &T {
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

fn deref_3() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));
}

fn deref_show(s: &str) {
    println!("{}", s);
}

fn deref_4() {
    let x = "t1";
    let y = Box::new("t2");
    let z = MyBox::new(String::from("t3"));

    deref_show(&x); // *(&x)                ->  *(&str)     -> str
    deref_show(&y); // *(*(&(Box<str>)))    ->  *(Box<str>) ->
                    //  str (as box has deref trait)
    deref_show(&z); // Just chained derefs until item achieved (auto deref)
//  z.drop(); Not allowed explicit call
    deref_show(&((*z)[..]));

    drop(z); // not our drop, std lib drop, early drop
    println!("before end");
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox: dropping...");
    }
}

pub fn box_lib_runner() {
    deref_1();
    deref_2();
    deref_3();
    deref_4();
}