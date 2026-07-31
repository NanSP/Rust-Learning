#[derive(Debug)]
enum TypeAddressIp {
    V4,
    V6,
}

fn simple_example_enum() {
    let type1 = TypeAddressIp::V4;
    let type2: TypeAddressIp;

    type2 = TypeAddressIp::V6;

    println!("\n==SIMPLE EXAMPLE ENUM==");
    println!("Type1= {:?}  Type2= {:?}", type1, type2);
}

#[derive(Debug)]
struct AddressIp {
    types: TypeAddressIp,
    address: String,
}

fn example_with_struct() {
    let home = AddressIp {
        types: TypeAddressIp::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = AddressIp {
        types: TypeAddressIp::V6,
        address: String::from("::1"),
    };

    println!("\n---EXAMPLE WITH STRUCT---");
    println!("home {:?}  loopback {:?}", home, loopback);
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn example_with_enum() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("\n---EXAMPLE WITH ENUM---");
    println!("home: {:?}   loopback {:?}", home, loopback);
}

#[derive(Debug)]
enum IpAddrDif {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn example_with_enum2() {
    let home = IpAddrDif::V4(127, 0, 0, 1);
    let loopback = IpAddrDif::V6(String::from("::1"));

    println!("\n--EXAMPLE WITH ENUM 2--");
    println!("home {:?}  loopback {:?}", home, loopback);
}

fn main() {
    simple_example_enum();
    example_with_struct();
    example_with_enum();
    example_with_enum2();
}
