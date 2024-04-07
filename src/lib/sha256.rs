use std::ptr;

pub const SHA256_BLOCK_SIZE: usize = 32;

/**************************** DATA TYPES ****************************/
type BYTE = u8;
// 8-bit byte
type WORD = u32;  // 32-bit word

pub struct Sha256Ctx {
    data: [BYTE; 64],
    datalen: WORD,
    bitlen: u64,
    state: [WORD; 8],
}
impl Sha256Ctx {
	pub fn new() -> Sha256Ctx {
		return Sha256Ctx {
			bitlen:0,
			state: [0; 8],
			datalen: 0,
			data: [0; 64]
		}
	}
}

/****************************** MACROS ******************************/

fn ch(x: u32, y: u32, z: u32) -> u32 {
    return (x & y) ^ (!x & z)
}
fn maj(x: u32, y: u32, z:u32) -> u32 {
    return ((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z))
}
fn ep0(x: u32) -> u32 {
    return x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22);
}
fn ep1(x: u32) -> u32 {
    return x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}
fn sig0(x: u32) -> u32 {
    return x.rotate_right(7) ^ x.rotate_right(18) ^ ((x) >> 3)
}
fn sig1(x: u32) -> u32 {
    return x.rotate_right(17) ^ x.rotate_right(19) ^ ((x) >> 10)
}

/**************************** VARIABLES *****************************/
static K: [WORD; 64] = [
	0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
	0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
	0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
	0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
	0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
	0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
	0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
	0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
];

/*********************** FUNCTION DEFINITIONS ***********************/
fn sha256_transform(ctx: &mut Sha256Ctx)
{
	let mut data = ctx.data;
    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut t1, mut t2)
    : (WORD, WORD, WORD, WORD, WORD, WORD, WORD, WORD, WORD, WORD);
    let mut m: [WORD; 64] = [0; 64];

    let mut j = 0;
    for i in 0 .. 16 {
		m[i] = ((data[j] as u32) << 24)
			| ((data[j + 1] as u32) << 16)
			| ((data[j + 2] as u32) << 8)
			| (data[j + 3] as u32);
        j += 4;
    }

    for i in 16 .. 64 {
		let sig1result = sig1(m[i - 2]);
		let sig0result = sig0(m[i - 15]);
		m[i] = sig1result.wrapping_add(m[i - 7])
			.wrapping_add(sig0result)
			.wrapping_add(m[i - 16]);
    }


	a = ctx.state[0];
	b = ctx.state[1];
	c = ctx.state[2];
	d = ctx.state[3];
	e = ctx.state[4];
	f = ctx.state[5];
	g = ctx.state[6];
	h = ctx.state[7];

    for i in 0 .. 64 {
		t1 = h.wrapping_add(ep1(e))
			.wrapping_add(ch(e, f, g))
			.wrapping_add(K[i])
			.wrapping_add(m[i]);
		t2 = ep0(a).wrapping_add(maj(a, b, c));
		h = g;
		g = f;
		f = e;
		e = d.wrapping_add(t1);
		d = c;
		c = b;
		b = a;
		a = t1.wrapping_add(t2);
	}

	ctx.state[0] = ctx.state[0].wrapping_add(a);
	ctx.state[1] = ctx.state[1].wrapping_add(b);
	ctx.state[2] = ctx.state[2].wrapping_add(c);
	ctx.state[3] = ctx.state[3].wrapping_add(d);
	ctx.state[4] = ctx.state[4].wrapping_add(e);
	ctx.state[5] = ctx.state[5].wrapping_add(f);
	ctx.state[6] = ctx.state[6].wrapping_add(g);
	ctx.state[7] = ctx.state[7].wrapping_add(h);
}

pub fn sha256_init(ctx: &mut Sha256Ctx) {
    ctx.datalen = 0;
    ctx.bitlen = 0;
    ctx.state[0] = 0x6a09e667;
	ctx.state[1] = 0xbb67ae85;
	ctx.state[2] = 0x3c6ef372;
	ctx.state[3] = 0xa54ff53a;
	ctx.state[4] = 0x510e527f;
	ctx.state[5] = 0x9b05688c;
	ctx.state[6] = 0x1f83d9ab;
	ctx.state[7] = 0x5be0cd19;
}

pub fn sha256_update(ctx: &mut Sha256Ctx, data: &[BYTE], len: usize)
{
    for i in 0..len {
        ctx.data[ctx.datalen as usize] = data[i];
        ctx.datalen += 1;
        if (ctx.datalen == 64) {
            sha256_transform(&mut *ctx);
            ctx.bitlen += 512;
            ctx.datalen = 0;
        }
    }
}

pub fn sha256_final(ctx: &mut Sha256Ctx, hash: &mut [BYTE])
{
	let mut  i: WORD = ctx.datalen;

	// Pad whatever data is left in the buffer.
	if (ctx.datalen < 56) {
		ctx.data[i as usize] = 0x80;
        i += 1;
		while i < 56 {
            ctx.data[i as usize] = 0x00;
            i +=1;
        }
	}
	else {
        ctx.data[i as usize] = 0x80;
        i += 1;
        while i < 64 {
            ctx.data[i as usize] = 0x00;
            i += 1;
        }
		sha256_transform(&mut *ctx);
        ctx.data[0..56].fill(0);
	}

	// Append to the padding the total message's length in bits and transform.
	ctx.bitlen += (ctx.datalen * 8) as u64;
	ctx.data[63] = (ctx.bitlen & 0xff) as BYTE;
	ctx.data[62] = (ctx.bitlen >> 8) as BYTE;
	ctx.data[61] = (ctx.bitlen >> 16) as BYTE;
	ctx.data[60] = (ctx.bitlen >> 24) as BYTE;
	ctx.data[59] = (ctx.bitlen >> 32) as BYTE;
	ctx.data[58] = (ctx.bitlen >> 40) as BYTE;
	ctx.data[57] = (ctx.bitlen >> 48) as BYTE;
	ctx.data[56] = (ctx.bitlen >> 56) as BYTE;
	sha256_transform(&mut *ctx);

	// Since this implementation uses little endian byte ordering and SHA uses big endian,
	// reverse all the bytes when copying the final state to the output hash.
	for i in 0 .. 4 {
		hash[i]      = ((ctx.state[0] >> (24 - i * 8)) & 0x000000ff) as BYTE;
		hash[i + 4]  = ((ctx.state[1] >> (24 - i * 8)) & 0x000000ff) as BYTE;
		hash[i + 8]  = ((ctx.state[2] >> (24 - i * 8)) & 0x000000ff) as BYTE;
		hash[i + 12] = ((ctx.state[3] >> (24 - i * 8)) & 0x000000ff) as BYTE;
		hash[i + 16] = ((ctx.state[4] >> (24 - i * 8)) & 0x000000ff) as BYTE;
		hash[i + 20] = ((ctx.state[5] >> (24 - i * 8)) & 0x000000ff) as BYTE;
		hash[i + 24] = ((ctx.state[6] >> (24 - i * 8)) & 0x000000ff) as BYTE;
		hash[i + 28] = ((ctx.state[7] >> (24 - i * 8)) & 0x000000ff) as BYTE;
	}
}