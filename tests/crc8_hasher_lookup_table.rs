mod base;
use self::base::SMBUS_PEC_LOOKUP_TABLE as LOOKUP_TABLE;
use core::hash::Hasher;
use embedded_crc_macros::crc8_hasher_lookup_table;

crc8_hasher_lookup_table!(struct SmbusPec, 0, "SMBus Packet Error Code");

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

#[test]
fn new_is_the_same_as_default() {
    assert_eq!(SmbusPec::new(), SmbusPec::default());
}

#[test]
fn can_be_copied() {
    let pec = SmbusPec::new();
    let copy = pec;
    assert_eq!(pec, copy);
}

#[test]
fn can_be_debug_printed() {
    println!("{:?}", SmbusPec::new());
}

#[test]
fn macro_can_be_used_within_function() {
    crc8_hasher_lookup_table!(struct H, 2, "hasher");
    let _ = H::new();
}
