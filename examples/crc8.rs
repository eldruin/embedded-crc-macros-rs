use embedded_crc_macros::crc8;

crc8!(pec, 7 /* x^8+x^2+x+1 */, 0, "SMBus Packet Error Code");

const ADDRESS: u8 = 0x5A;
const REGISTER: u8 = 0x06;

fn main() {
    let crc = pec(&[ADDRESS << 1, REGISTER, 0xAB, 0xCD]);
    println!("PEC: {}", crc); // prints 95
}
