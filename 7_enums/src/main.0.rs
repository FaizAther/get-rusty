// Enums

#[derive(Debug)]
enum IpAddrT {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrT,
    addr: String
}

fn ip_kind() {
    let ip4_t: IpAddrT = IpAddrT::V4;
    let ip6_t: IpAddrT = IpAddrT::V6;

    let ipv4: IpAddr = IpAddr {
        kind: ip4_t,
        addr: String::from("127.0.0.1")
    };

    let ipv6: IpAddr = IpAddr {
        kind: ip6_t,
        addr: String::from("127.0.0.1")
    };

    println!("{:?}", ipv4);
    println!("{:?}", ipv6);
}

fn main() {
    ip_kind();
    println!("Hello, world!");
}
