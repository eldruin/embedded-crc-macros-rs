mod base;
use base::SMBUS_PEC_LOOKUP_TABLE as LOOKUP_TABLE;
use embedded_crc_macros::crc8;

crc8!(smbus_pec, 7, 0, "SMBus Packet Error Code");

#[test]
fn check_pec_table() {
    for (i, expected) in LOOKUP_TABLE.iter().enumerate() {
        assert_eq!(smbus_pec(&[i as u8]), *expected);
    }
}

#[test]
fn check_pec_array() {
    const ADDR: u8 = 0x5A;
    const REGISTER: u8 = 0x06;
    assert_eq!(smbus_pec(&[ADDR << 1, REGISTER, 0xAB, 0xCD]), 95);
    assert_eq!(
        smbus_pec(&[ADDR << 1, REGISTER, (ADDR << 1) + 1, 38, 58]),
        102
    );
    assert_eq!(
        smbus_pec(&[ADDR << 1, REGISTER, (ADDR << 1) + 1, 107, 58]),
        212
    );
    assert_eq!(
        smbus_pec(&[ADDR << 1, REGISTER, (ADDR << 1) + 1, 97, 58]),
        86
    );
    assert_eq!(
        smbus_pec(&[ADDR << 1, REGISTER, (ADDR << 1) + 1, 225, 57]),
        233
    );
}
