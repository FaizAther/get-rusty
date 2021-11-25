// Life

// Concept 1

fn life1() {
	let _r: &i32;

	{
		let _x: i32 = 5;
		_r = &_x;
	}
    //println!("r: {}", r); not allowed x has died
}

// Concept 2

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x }
    else { y }
}

fn life2() {
    let st1 = String::from("abc");
    let res;

    //{
        let st2 = String::from("xy");
        res = longest(st1.as_str(), st2.as_str());
    //}

    println!("{}", res);
}

// Concept 3
#[derive(Debug)]
struct ImpExt<'a> {
    part: &'a str,
}

fn life3() {
    let novel = String::from("Call me Ishmael. Some years ago... In the");
    let sent_1 = novel.split('.').next().unwrap_or("Not found");
    for ss in novel.split('.') {
        println!("{}", ss);
    }
    let i = ImpExt {
        part: sent_1
    };
    println!("{:?}", i);
}

fn first_word<'a>(s: &'a str, q:&'a str) -> &'a str {
    if s.len() > q.len() { return s; }
    else { return q; }
}

fn life4() {
    let word = first_word("s: &'a str", "q: &'a str");
    println!("{}", word);
}

impl<'a> ImpExt<'a> {
    fn return_part(&'a self, announcement: &'a str) -> &'a str {
        println!("Attention please {}", announcement);
        return self.part;
    }
}

fn life5() {
    let i = ImpExt {
        part: "HelloWrold"
    };
    println!("part: {}", i.return_part("I got the part"));
}

fn life6() {
    let s: &'static str = "I have a static life";
    println!("{}", s);
}

use std::fmt::Display;

fn longest_annon<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Annoncement {}", ann);
    if x.len() > y.len() { return x; }
    else { return y; }
}

fn life7() {
    let check = longest_annon("x: &'a str", "y: &'a str", "ann: T");
    println!("{}", check);
}

fn main() {
    life1();
    life2();
    life3();
    life4();
    life5();
    life6();
    life7();
}
