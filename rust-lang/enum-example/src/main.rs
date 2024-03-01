#[derive(Debug)]
enum IpAddress {
    V4,
    V6,
}

enum IpAddressWithData {
    V4(String),
    v6(String),
}

enum IpAddressWithPieces {
    V4(u32, u32, u32, u32),
    V6(u32, u32, u32, u32, u32, u32),
}

enum Response {
    Success(String, u32),
    Error(String, u32, Vec<String>),
}

fn main() {
    let address1 = IpAddress::V4;
    println!("address-1 is {:?}", address1);
}
