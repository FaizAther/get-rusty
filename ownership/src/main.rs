fn main() {
    println!("Hello, world!");

    let x = 5;
    let _y = x;

    let s1 = String::from("hello1");
    let s2 = s1.clone();

    println!("{}, world", s1);

    takes_own(s2);

    // println!("{}", s2);

    let s3 = gives_own();
    println!("{}",  s3);

    println!("{}", take_give_own(s3));
}

fn takes_own(some_string: String)
{
    println!("{}", some_string)
}

fn gives_own() -> String {
    let some_string = String::from("hello2");

    some_string
}

fn take_give_own(some_string: String) -> String {
    some_string
}