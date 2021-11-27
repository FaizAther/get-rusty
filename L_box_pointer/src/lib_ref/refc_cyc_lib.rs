// Cyclic memleaks

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum MyList<T>
//    where T: Copy + Clone
{
    Cons(T, RefCell<Rc<MyList<T>>>),
    Nil
}

trait ListOps<T>
//    where T: Copy + Clone
{
    fn tail(&self) -> Option<&RefCell<Rc<MyList<T>>>>;

    fn head(&self) -> Option<&T>;

    fn nil() -> RefCell<Rc<MyList<T>>>;

    fn new(_: T) -> RefCell<Rc<MyList<T>>>;

    fn add(_: RefCell<Rc<MyList<T>>>, _: T) -> RefCell<Rc<MyList<T>>>;

    fn clone(_: T, _: &Rc<MyList<T>>) -> RefCell<Rc<MyList<T>>>;
}

//impl<T> Copy for MyList<T> {
//
//}
//
//impl<T> Clone for MyList<T> {
//    fn clone(&self) -> MyList<T> {
//        return *self;
//    }
//}

impl<T> ListOps<T> for MyList<T> 
//    where T: Copy + Clone
{
    fn tail(&self) -> Option<&RefCell<Rc<MyList<T>>>> {
        return match self {
            MyList::Nil => None,
            MyList::Cons(_, tail) => Some(tail),
        };
    }

    fn head(&self) -> Option<&T> {
        return match self {
            MyList::Nil => None,
            MyList::Cons(item, _) => Some(item),
        }
    }

    fn nil() -> RefCell<Rc<MyList<T>>> {
        return RefCell::new(Rc::new(MyList::<T>::Nil));
    }

    fn new(item: T) -> RefCell<Rc<MyList<T>>> {
        return RefCell::new(Rc::new(MyList::<T>::Cons(item, MyList::nil())));
    }

    fn add(list: RefCell<Rc<MyList<T>>>, item: T) -> RefCell<Rc<MyList<T>>> {
       return RefCell::new(Rc::new(MyList::Cons(item, list)));
    }
    
    fn clone(item: T, list: &Rc<MyList<T>>) -> RefCell<Rc<MyList<T>>> {
        return RefCell::new(Rc::new(MyList::Cons(item,
            RefCell::new(Rc::clone(list)))));
    }
}

use std::ops::Deref;

fn run1() {
    let nil = MyList::<u64>::nil();
    let l1 = MyList::<u64>::add(nil, 30003);

    let l2 = MyList::<u64>::clone(10001, &l1.borrow());

    println!("{:?}", MyList::tail(l1.borrow().deref()));
    println!("{:?}", l1.borrow().deref().tail());
    println!("{:?}", l1.borrow().deref().head());
    println!("{:?}", MyList::head(l1.borrow().deref()));
    println!("{:?}", l1);

    println!("{:?}", MyList::tail(l2.borrow().deref()));
    println!("{:?}", l2.borrow().deref().tail());
    println!("{:?}", l2.borrow().deref().head());
    println!("{:?}", l2);
}

pub fn refc_cyc_lib_runner() {
    println!("refc_cyc_lib_runner");
    run1();
} 
