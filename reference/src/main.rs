// Concepts pointers

fn concept_1() {
    let s1 = String::from("hello");
    let (s2, len) = calc_len_1(s1);
    println!("{}, {}", s2, len);
}

fn calc_len_1(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn concept_2() {
    let s1 = String::from("hello");
    let (s2, len) = calc_len_2(&s1);
    println!("world {}, {}", s2, len);
}

fn calc_len_2(s: &String) -> (&String, usize) {
    // s.push_str("ops");
    let len = s.len();
    (s, len)
}

//  Data Race
fn concept_3() {
   let mut s:String = String::from("Hello");

   let r1:&String = &s;
   let r2:&String = &s;

//   let r3:&String = &mut s;

   println!("{}, {}", r1, r2);

// Scope of immutable reference ends here so can create \
// here

   let r3:&String = &mut s;
   println!("{}", r3);
}

fn main() {
    concept_1();
    concept_2();
    concept_3();
}
