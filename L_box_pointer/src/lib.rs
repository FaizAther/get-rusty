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

pub fn deref() {
    deref_1();
    deref_2();
    deref_3();
}
