extern crate alloc;

use ruint::aliases::U256;
use ruint::{uint, Bits, Uint};

pub type FpVesta2 = Fp<Fp256Param, 256, 4>;

pub struct Fp<P: FpParam<BITS, LIMBS>, const BITS: usize, const LIMBS: usize>(
    Uint<BITS, LIMBS>,
    core::marker::PhantomData<P>,
);

pub struct Fp256Param;

impl FpParam<256, 4> for Fp256Param {
    const MODULUS: U256 =
        uint!(28948022309329048855892746252171976963363056481941647379679742748393362948097_U256);
    const GENERATOR: U256 = uint!(5_U256);
}

pub trait FpParam<const BITS: usize, const LIMBS: usize> {
    const MODULUS: Uint<BITS, LIMBS>;
    const GENERATOR: Uint<BITS, LIMBS>;
}

fn main() {
    let layout = alloc::alloc::Layout::from_size_align(17, 1).unwrap();
    let ptr = unsafe { alloc::alloc::alloc(layout) };

    let uint = uint!(5_U256);
    let bits = into_bits(uint);
    let uint1 = bits.into_inner();

    print!("Hello, World!");
}

const fn into_bits<const BITS: usize, const LIMBS: usize>(
    uint: Uint<BITS, LIMBS>,
) -> Bits<BITS, LIMBS> {
    Bits::from_limbs(uint.into_limbs())
}
