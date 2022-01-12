const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

///
// TODO: There is a log_2 experimental function on nightly. Waiting for it...
//  https://github.com/rust-lang/rust/issues/70887
pub fn log2(x: i32) -> u32 {
    assert!(x > 0);
    num_bits::<i32>() as u32 - x.leading_zeros() - 1
}
