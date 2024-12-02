const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;

const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &'static str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings<'static> = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};

struct BitCalculator<const P: BitsNStrings> {}

fn main() {
    println!("{} {}", BITS_N_STRINGS.mybits[0], BITS_N_STRINGS.mybits[1]);
    println!("{}", BITS_N_STRINGS.mystring);
}
