
#[derive(Debug)]
enum EnderecoIp {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let local = EnderecoIp::V4(127, 0, 0, 1);

    let loopback = EnderecoIp::V6(String::from("::1"));

    println!("Endereço IP local: {:?}", local);
    println!("Endereço IP loopback: {:?}", loopback);

}
