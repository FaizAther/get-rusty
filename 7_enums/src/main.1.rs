// Enums

#[derive(Debug)]
enum IpAddrT {
    V4(String),
    V6(String)
}

fn ip_kind() {
    let ip4_t: IpAddrT = IpAddrT::V4(String::from("127.0.0.1"));
    let ip6_t: IpAddrT = IpAddrT::V6(String::from("::1"));

    println!("{:?}", ip4_t);
    println!("{:?}", ip6_t);
}

fn main() {
    ip_kind();
    println!("Hello, world!");
}
