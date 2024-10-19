use ruint::aliases::U256;
use ruint::{uint, Uint};

pub type FpVesta2 = Fp<Fp256Param>;

pub struct Fp<P: FpParam>(
    Uint<{ <P as FpParam>::BITS }, { <P as FpParam>::LIMBS }>,
    core::marker::PhantomData<P>,
);
pub struct Fp256Param;
impl FpParam for Fp256Param {
    const GENERATOR: U256 = uint!(5_U256);
    const MODULUS: U256 =
        uint!(28948022309329048855892746252171976963363056481941647379679742748393362948097_U256);
    const BITS: usize = 256;
    const LIMBS: usize = 4;
}

pub trait FpParam {
    const MODULUS: Uint<{ <Self as FpParam>::BITS }, { <Self as FpParam>::LIMBS }>;
    const GENERATOR: Uint<{ <Self as FpParam>::BITS }, { <Self as FpParam>::LIMBS }>;
    const BITS: usize;
    const LIMBS: usize;
}

fn main() {
    print!("Hello, World!");
}
