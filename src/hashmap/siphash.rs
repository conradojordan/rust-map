const COMPRESSION_ROUNDS: u8 = 2;
const FINALIZATION_ROUNDS: u8 = 4;

pub trait SipHasher {
    // fn u64_iter(&self) -> Iter;

    fn sip_hash(&self, key: u128) -> u64 {
        // Transform to little_endian
        let le_key = key.swap_bytes();
        let k: [u64; 2] = [(le_key >> 64) as u64, le_key as u64];
        let mut v: [u64; 4] = [
            0x736f6d6570736575 ^ k[0],
            0x646f72616e646f6d ^ k[1],
            0x6c7967656e657261 ^ k[0],
            0x7465646279746573 ^ k[1],
        ];
        // let m = *self as u64;
        let m = 0x0100000000000061;
        println!("b = {}", m);
        let what: u64 = 1441151882442859620;
        println!("le_what = {:#018x}", what.swap_bytes());
        sip_chunk(&mut v, m);

        v[2] ^= 0xff as u64;

        for i in 0..v.len() {
            println!("v[{}] = {}", i, v[i]);
        }
        for _ in 0..FINALIZATION_ROUNDS {
            sip_round(&mut v);
        }

        let result = v[0] ^ v[1] ^ v[2] ^ v[3];
        println!("Result = {}", result);
        result
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

impl SipHasher for i32 {}
