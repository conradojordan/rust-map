/*
    Author: Conrado Jordan
    Date: 2023
    Implementation of the SipHash hashing algorithm (https://en.wikipedia.org/wiki/SipHash)
    created by Jean-Philippe Aumasson and Daniel J. Bernstein.
    Reference paper used for the implementation: https://eprint.iacr.org/2012/351.pdf.
    Public domain C Reference implementation can be found at https://github.com/veorq/SipHash
*/
use crate::hashmap::MapKeyHasher;

const COMPRESSION_ROUNDS: u8 = 2;
const FINALIZATION_ROUNDS: u8 = 4;

pub trait SipHasher {
    // Divides instance of the implementingtype into chunks of 64 bits (a Vec<u64>).
    // The instance of b bytes should be divided into w = [(b + 1)/8] > 0 64-bit little-endian words
    // m0, . . . , mw−1 where mw−1 includes the last 0 through 7 bytes of m followed by null bytes and ending
    // with a byte encoding the positive integer (b mod 256).
    fn u64_chunks(&self) -> Vec<u64>;

    fn sip_hash(&self, key: u128) -> u64 {
        // To transform key to little_endian use key.swap_bytes();
        // Not needed as a random generated key is being used (its random bytes anyway)
        let k: [u64; 2] = [(key >> 64) as u64, key as u64];
        let mut v: [u64; 4] = [
            0x736f6d6570736575 ^ k[0],
            0x646f72616e646f6d ^ k[1],
            0x6c7967656e657261 ^ k[0],
            0x7465646279746573 ^ k[1],
        ];

        for chunk in self.u64_chunks() {
            sip_chunk(&mut v, chunk);
        }

        v[2] ^= 0xff as u64;

        for _ in 0..FINALIZATION_ROUNDS {
            sip_round(&mut v);
        }

        v[0] ^ v[1] ^ v[2] ^ v[3]
    }
}

fn sip_round(v: &mut [u64; 4]) {
    // One round of the SipHash, called SipRound
    v[0] = v[0].wrapping_add(v[1]);
    v[1] = v[1].rotate_left(13);
    v[1] ^= v[0];
    v[0] = v[0].rotate_left(32);
    v[2] = v[2].wrapping_add(v[3]);
    v[3] = v[3].rotate_left(16);
    v[3] ^= v[2];
    v[0] = v[0].wrapping_add(v[3]);
    v[3] = v[3].rotate_left(21);
    v[3] ^= v[0];
    v[2] = v[2].wrapping_add(v[1]);
    v[1] = v[1].rotate_left(17);
    v[1] ^= v[2];
    v[2] = v[2].rotate_left(32);
}

fn sip_chunk(v: &mut [u64; 4], m: u64) {
    // Updates the states vector by applying `COMPRESSION_ROUNDS` SipRounds
    // with the given 64 bits chunk (m)
    v[3] ^= m;

    for _ in 0..COMPRESSION_ROUNDS {
        sip_round(v);
    }

    v[0] ^= m;
}

// Implementation of the MapKeyHasher trait for Rust types
impl MapKeyHasher for bool {
    fn hash(&self, _hash_seed: u128) -> u64 {
        if *self {
            0 as u64
        } else {
            1 as u64
        }
    }
}

impl SipHasher for u128 {
    fn u64_chunks(&self) -> Vec<u64> {
        vec![(*self >> 64) as u64, *self as u64, 0x1000000000000000]
    }
}

impl MapKeyHasher for u128 {
    fn hash(&self, hash_seed: u128) -> u64 {
        self.sip_hash(hash_seed)
    }
}

impl SipHasher for i128 {
    fn u64_chunks(&self) -> Vec<u64> {
        vec![(*self >> 64) as u64, *self as u64, 0x1000000000000000]
    }
}

impl MapKeyHasher for i128 {
    fn hash(&self, hash_seed: u128) -> u64 {
        self.sip_hash(hash_seed)
    }
}

// Macro to implement MapKeyHasher using the SipHasher trait
macro_rules! implement_sip {
    ($elem:ty) => {
        impl SipHasher for $elem {
            fn u64_chunks(&self) -> Vec<u64> {
                vec![(*self as u64).swap_bytes(), 0x800000000000000]
            }
        }

        impl MapKeyHasher for $elem {
            fn hash(&self, hash_seed: u128) -> u64 {
                self.sip_hash(hash_seed)
            }
        }
    };
}

implement_sip!(i8);
implement_sip!(u8);
implement_sip!(i16);
implement_sip!(u16);
implement_sip!(i32);
implement_sip!(u32);
implement_sip!(i64);
implement_sip!(u64);
implement_sip!(isize);
implement_sip!(usize);
