mod base;
use base::SMBUS_PEC_LOOKUP_TABLE as LOOKUP_TABLE;
use core::hash::Hasher;
use embedded_crc_macros::crc8_hasher;

crc8_hasher!(SmbusPec, 7, 0, "SMBus Packet Error Code");

#[test]
fn check_pec_table() {
    for (i, expected) in LOOKUP_TABLE.iter().enumerate() {
        let mut hasher = SmbusPec::new();
        hasher.write(&[i as u8]);
        assert_eq!(hasher.finish(), *expected as u64);
    }
}

#[test]
fn check_pec_array() {
    const ADDR: u8 = 0x5A;
    const REGISTER: u8 = 0x06;
    let mut hasher = SmbusPec::new();
    hasher.write(&[ADDR << 1, REGISTER, 0xAB, 0xCD]);
    assert_eq!(hasher.finish(), 95);
}
