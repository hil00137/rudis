/* Get random bytes, attempts to get an initial seed from /dev/urandom and
 * the uses a one way hash function in counter mode to generate a random
 * stream. However if /dev/urandom is not available, a weaker seed is used.
 *
 * This function is not thread safe, since the state is global. */
use std::fs::File;
use std::io::Read;

use libc::{getpid, gettimeofday, timeval};

use crate::lib::sha256::{SHA256_BLOCK_SIZE, sha256_final, sha256_init, sha256_update, Sha256Ctx};

pub fn get_random_bytes(p: &mut [u8; 16]) {
    let mut len = p.len();
    /* Global state. */
    let mut seed_initialized = false;
    let mut seed: [u8; 64] = [0; 64]; /* 512 bit internal block size. */
    let mut counter: u64 = 0; /* The counter we hash with the seed. */

    if (!seed_initialized) {
        /* Initialize a seed and use SHA1 in counter mode, where we hash
         * the same seed with a progressive counter. For the goals of this
         * function we just need non-colliding strings, there are no
         * cryptographic security needs. */
        let fp = File::open("/dev/urandom");
        match fp {
            Ok(mut file) => {
                if file.read_exact(&mut seed).is_err() {
                    // 파일에서 읽기 실패 시 또는 읽은 바이트 수가 seed의 크기와 다를 경우
                    fallback_seed(&mut seed);
                } else {
                    seed_initialized = true;
                }
            }
            Err(_) => fallback_seed(&mut seed),
        }
    }

    let mut position = 0;
    while len != 0 {
        /* This implements SHA256-HMAC. */
        let mut digest= [0u8; SHA256_BLOCK_SIZE];
        let mut kxor = [0u8; 64];
        let copylen: usize;
        if len > SHA256_BLOCK_SIZE {
            copylen = SHA256_BLOCK_SIZE
        } else {
            copylen = len
        }

        /* IKEY: key xored with 0x36. */
        kxor.copy_from_slice(&seed);
        for i in 0 .. kxor.len() {
            kxor[i] ^= 0x36
        }

        /* Obtain HASH(IKEY||MESSAGE). */
        let mut ctx: Sha256Ctx = Sha256Ctx::new();
        sha256_init(&mut ctx);
        sha256_update(&mut ctx, &kxor, kxor.len());
        let counter_bytes = counter.to_ne_bytes();
        sha256_update(&mut ctx, &counter_bytes, counter_bytes.len());
        sha256_final(&mut ctx, &mut digest);

        /* OKEY: key xored with 0x5c. */
        kxor.copy_from_slice(&seed);
        for i in 0..kxor.len() {
            kxor[i] ^= 0x5c;
        }

        /* Obtain HASH(OKEY || HASH(IKEY||MESSAGE)). */
        sha256_init(&mut ctx);
        sha256_update(&mut ctx, &kxor, kxor.len());
        sha256_update(&mut ctx, &digest,SHA256_BLOCK_SIZE);
        sha256_final(&mut ctx,&mut digest);

        /* Increment the counter for the next iteration. */
        counter += 1;


        p[position..copylen].copy_from_slice(&digest[position..copylen]);
        len -= copylen;
        position += copylen;
    }
}

fn fallback_seed(seed: &mut [u8; 64]) {
    for j in 0..seed.len() {
        unsafe {
            let mut tv = timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            gettimeofday(&mut tv, std::ptr::null_mut());
            let pid = getpid();
            seed[j] = (tv.tv_sec as u8) ^ (tv.tv_usec as u8) ^ (pid as u8);
        }
    }
}