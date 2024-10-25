const RES: usize = div_64::<128>();
fn main() {
    println!("Hello, World!");

    let arr = [0; RES];
    println!("{:?}", arr);
}

/////////////////////////

#[allow(dead_code, path_statements, clippy::no_effect)]
pub(crate) const fn smaller_than<const N: usize, const MAX: usize>() {
    Assert::<N, MAX>::LESS;
}

#[allow(dead_code, path_statements, clippy::no_effect)]
pub(crate) const fn greater_than_eq<const N: usize, const MIN: usize>() {
    Assert::<N, MIN>::GREATER_EQ;
}

#[allow(dead_code, path_statements, clippy::no_effect)]
pub(crate) const fn greater_than_eq_0<const N: usize>() {
    Assert::<N, 0>::GREATER_EQ;
}

#[allow(dead_code, path_statements, clippy::no_effect)]
pub(crate) const fn greater_than_0<const N: usize>() {
    Assert::<N, 0>::GREATER;
}

#[allow(dead_code, path_statements, clippy::no_effect)]
pub(crate) const fn greater_than_1<const N: usize>() {
    Assert::<N, 1>::GREATER;
}

#[allow(dead_code, path_statements, clippy::no_effect)]
pub(crate) const fn power_of_two<const N: usize>() {
    Assert::<N, 0>::GREATER;
    Assert::<N, 0>::POWER_OF_TWO;
}

#[allow(dead_code, path_statements, clippy::no_effect)]
pub(crate) const fn div_64<const N: usize>() -> usize {
    Assert::<N, 0>::DIV_64
}

#[allow(dead_code)]
/// Const assert hack
pub struct Assert<const L: usize, const R: usize>;

#[allow(dead_code)]
impl<const L: usize, const R: usize> Assert<L, R> {
    /// Const assert hack
    pub const GREATER_EQ: usize = L - R;

    /// Const assert hack
    pub const LESS_EQ: usize = R - L;

    /// Const assert hack
    #[allow(clippy::erasing_op)]
    pub const NOT_EQ: isize = 0 / (R as isize - L as isize);

    /// Const assert hack
    pub const EQ: usize = (R - L) + (L - R);

    /// Const assert hack
    pub const GREATER: usize = L - R - 1;

    /// Const assert hack
    pub const LESS: usize = R - L - 1;

    /// Const assert hack
    pub const POWER_OF_TWO: usize = 0 - (L & (L - 1));

    /// Const assert hack
    pub const DIV_64: usize = L.div_ceil(64);
}
