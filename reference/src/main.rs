// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = calc_len(s1);
//     println!("{}, {}", s2, len);
// }

// fn calc_len(s: String) -> (String, usize) {
//     let len = s.len();
//     (s, len)
// }
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calc_len(&s1);
    println!("{}, {}", s2, len);
}

fn calc_len(s: &String) -> (&String, usize) {
    // s.push_str("ops");
    let len = s.len();
    (s, len)
}
