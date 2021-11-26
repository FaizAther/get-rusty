// Smart Pointers

#[derive(Debug)]
enum MyList<T> {
    Cons(T, Box<MyList<T>>),
    Nil
}

fn main() {
    let b: Box<u32> = Box::new(5);
    println!("{}", b);

    let nil: Box<MyList<i32>> = Box::new(MyList::<i32>::Nil);
    let list = Box::new(MyList::<i32>::Cons(44, nil));
    let list1 = Box::new(MyList::<i32>::Cons(55, list));
    println!("{:?}", list1);
}
