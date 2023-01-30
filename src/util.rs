// Get two non-overlapping ranges starting at i & j, both with length len
pub fn get_both_ranges<T>(
    vector: &mut [T],
    i: usize,
    j: usize,
    len: usize,
) -> (&mut [T], &mut [T]) {
    debug_assert_ne!(i, j);
    debug_assert!(i + len <= vector.len());
    debug_assert!(j + len <= vector.len());
    if i < j {
        debug_assert!(i + len <= j);
        let (first, last) = vector.split_at_mut(j);
        return (&mut first[i..(i + len)], &mut last[0..len]);
    } else {
        debug_assert!(j + len <= i);
        let (first, last) = vector.split_at_mut(i);
        return (&mut last[0..len], &mut first[j..(j + len)]);
    }
}

pub fn get_both_indices<T>(vector: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    debug_assert_ne!(i, j);
    debug_assert!(i < vector.len());
    debug_assert!(j < vector.len());
    if i < j {
        let (first, last) = vector.split_at_mut(j);
        return (&mut first[i], &mut last[0]);
    } else {
        let (first, last) = vector.split_at_mut(i);
        return (&mut last[0], &mut first[j]);
    }
}

// This should eventually become <https://doc.rust-lang.org/std/primitive.u64.html#method.div_ceil>
// when stabilized.
// The result known to not overflow from elsewhere.
pub fn int_div_ceil(num: u64, denom: u64) -> u32 {
    if denom==0 {panic!("called int_div_ceil on {} {}", num, denom)}
    if num%denom == 0 {(num/denom) as u32}
    else {(num/denom  + 1) as u32}
}
