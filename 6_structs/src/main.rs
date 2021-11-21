// Struct

struct User {
    id: u8,
    username: String,
    email: String,
    signin_count: u64,
    active: bool    
}

fn show_user(user: &User) {
    println!("User:");
    println!("\t{}", user.id);
    println!("\t{}", user.username);
    println!("\t{}", user.email);
    println!("\t{}", user.signin_count);
    println!("\t{}", user.active);
}

fn main() {
    let u1 = User{
        id: 5,
        username: String::from("John"),
        email: String::from("john@jesus.com"),
        signin_count: 2,
        active: false
    };
    show_user(&u1);
}
