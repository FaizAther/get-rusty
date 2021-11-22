// Struct

use std::fmt;

struct User {
    id: u8,
    username: String,
    email: String,
    signin_count: u64,
    active: bool    
}

//fn build_user(id: &mut u64, name: String, email: String) -> User {
//    (*id) += 1;
//    return User {
//        id: (*id - 1) as u8,
//        username: name,
//        email: email,
//        signin_count: 0,
//        active: false
//    };
//}

fn build_user_basic(id: &mut u64, name: String, email: String, basic: &User) -> User {
    (*id) += 1;
    return User {
        id: (*id - 1) as u8,
        username: name,
        email: email,
        ..(*basic)
    };
}
fn show_user(user: &User) {
    println!("User:");
    println!("|\t{}", user.id);
    println!("|\t{}", user.username);
    println!("|\t{}", user.email);
    println!("|\t{}", user.signin_count);
    println!("|\t{}", user.active);
    println!("|_____");
}

fn user_runner() {
    let mut id: u64 = 0;
    let basic = User{
        id: !(0) as u8,
        username: String::from("basic"),
        email: String::from("@jesus.com"),
        signin_count: 0,
        active: false
    };
//    id += 1;
    show_user(&basic);
    let u2 = build_user_basic(&mut id, String::from("jack"), String::from("jack@jesus.com"), &basic);
    show_user(&u2);
    println!("id: {}", id);
}

struct Rect {
    width: u32,
    height: u32
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rect: (width: {}, height: {})", self.width, self.height)
    }
}

fn area(rect: &Rect) -> u64 {
    return (rect.width * rect.height) as u64;
}

fn area_runner() {
    let rect: Rect = Rect{width: 5, height: 10};
    let area: u64 = area(&rect);
    println!("{}, area: {}", rect, area);
}

fn main() {
    user_runner();
    area_runner();
}
