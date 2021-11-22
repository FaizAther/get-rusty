// Option

// enum Option<T> {
//     Some(T),
//     None
// }

fn main() {
    let some_num = Some(9);
    let _some_str = Some("a str");

    let none_num: Option<i32> = None;

    let res = some_num.unwrap() + 9;
    let non = none_num.unwrap_or_default() + 9;    // defaulting to zero

    println!("{}, {}", res, non);

    let non_err = none_num.unwrap() + 9;    // unwrap a none: runtime error

    println!("{}", non_err);

}
