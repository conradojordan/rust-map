const COMPRESSION_ROUNDS: u8 = 2;
const FINALIZATION_ROUNDS: u8 = 4;

pub trait SipHasher {
    fn u64_chunks(&self) -> Vec<u64>;

    fn sip_hash(&self, key: u128) -> u64 {
        // To transform key to little_endian do:
        // let le_key = key.swap_bytes();
        // Not needed if using a random generated key (its random bytes anyway)
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
    v[3] ^= m;

    for _ in 0..COMPRESSION_ROUNDS {
        sip_round(v);
    }

    v[0] ^= m;
}

impl SipHasher for i32 {
    fn u64_chunks(&self) -> Vec<u64> {
        vec![(*self as u64).swap_bytes(), 0x800000000000000]
    }
}

impl SipHasher for u32 {
    fn u64_chunks(&self) -> Vec<u64> {
        vec![(*self as u64).swap_bytes(), 0x800000000000000]
    }
}
