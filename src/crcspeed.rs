// typedef uint64_t (*crcfn64)(uint64_t, const void *, const uint64_t);
type CrcFn64 = fn(u64, *const u8, u64) -> u64;

/* Fill in a CRC constants table. */
fn crcspeed64little_init(crcfn: CrcFn64, mut table: [[u64; 256]; 8]) {
    let crc: u64;

    /* generate CRCs for all single byte sequences */
    for n in 0u8..=255 {
        let v: u8 = n;
        table[0][n] = crcfn(0, &v, 1);
    }

    /* generate nested CRC table for future slice-by-8 lookup */
    for n in 0u8..=255 {
        crc = table[0][n];
        for k in 1..8 {
            crc = table[0][crc & 0xff] ^ (crc >> 8);
            table[k][n] = crc;
        }
    }
}

/* Reverse the bytes in a 64-bit word. */
fn rev8(a: u64) -> u64 {
    a.swap_bytes()
}

/* This function is called once to initialize the CRC table for use on a
   big-endian architecture. */
fn crcspeed64big_init(crcfn: CrcFn64, mut big_table: [[u64; 256]; 8]) {
    /* Create the little endian table then reverse all the entries. */
    crcspeed64little_init(crcfn, big_table);
    for k in 0..8 {
        for n in 0..=255 {
            big_table[k][n] = rev8(big_table[k][n]);
        }
    }
}

/* Initialize CRC lookup table in architecture-dependent manner. */
pub fn crcspeed64native_init(func: CrcFn64, table: [[u64; 256]; 8]) {
    if cfg!(target_endian = "little") {
        crcspeed64little_init(func, table);
    } else {
        crcspeed64big_init(func, table);
    }
}