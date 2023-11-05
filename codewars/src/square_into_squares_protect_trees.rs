//https://www.codewars.com/kata/54eb33e5bc1a25440d000891/train/rust

fn decompose(n: i64) -> Option<Vec<i64>> {
    // your code
    unimplemented!()
}

#[test]
fn tests_decompose() {
    let exp = Some(vec![1, 3, 5, 8, 49]);
    assert_eq!(decompose(50), exp);
    let exp = Some(vec![2, 3, 5, 7, 43]);
    assert_eq!(decompose(44), exp);
    let exp = Some(vec![2, 5, 8, 34, 624]);
    assert_eq!(decompose(625), exp);
    let exp = Some(vec![3, 4]);
    assert_eq!(decompose(5), exp);
}
