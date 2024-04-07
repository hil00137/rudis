/* -------------------------- hash functions -------------------------------- */

use std::sync::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    static ref DICT_HASH_FUNCTION_SEED: Mutex<[u8; 16]> = Mutex::new([0; 16]);
}

pub fn dict_set_hash_function_seed(seed: &[u8]) {
    let mut dict_seed = DICT_HASH_FUNCTION_SEED.lock().unwrap();
    dict_seed.copy_from_slice(seed);
}