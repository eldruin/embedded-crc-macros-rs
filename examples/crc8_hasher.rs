use core::hash::Hasher;
use embedded_crc_macros::crc8_hasher;

crc8_hasher!(
    SmbusPec,
    7, /* x^8+x^2+x+1 */
    0,
    "SMBus Packet Error Code"
);

const ADDRESS: u8 = 0x5A;
const REGISTER: u8 = 0x06;

fn main() {
    let mut hasher = SmbusPec::new();
    hasher.write(&[ADDRESS << 1, REGISTER, 0xAB, 0xCD]);
    let pec = hasher.finish();
    println!("PEC: {}", pec); // prints 95
}
