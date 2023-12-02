type Bitset = u128;

fn bitset_has(s: Bitset, index: usize) -> bool {
    (s & (1 << index)) != 0
}

fn bitset_set(s: &mut Bitset, index: usize) {
    *s |= 1 << index;
}

fn bitset_clear(s: &mut Bitset, index: usize) {
    *s &= !(1 << index);
}

fn bitset_first_index_set(s: Bitset) -> usize {
    s.trailing_zeros() as usize
}

fn index_to_ab(n: usize, index: usize) -> (usize, usize) {
    (index/n, index%n)
}

fn is_antisymmetric(n: usize, s: Bitset) -> bool {
    for a in 0..n {
        for b in 0..n {
            if a == b {
                continue
            }
            if bitset_has(s, a*n + b) && bitset_has(s, b*n + a) {
                return false;
            }
        }
    }
    return true;
}

fn is_transitive(n: usize, s: Bitset) -> bool {
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                if bitset_has(s, a*n + b) && bitset_has(s, b*n + c) && !bitset_has(s, a*n + c) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn is_transitive_fast(n: usize, s: Bitset, addition_index: usize) -> bool {
    let (first, second) = index_to_ab(n, addition_index);
    // (a, first) and (first, second) -> (a, second)
    for a in 0..n {
        if bitset_has(s, a*n + first) && !bitset_has(s, a*n + second) {
            return false;
        }
    }
    // (first, second) and (second, c) and -> (first, c)
    for c in 0..n {
        if bitset_has(s, second*n + c) && !bitset_has(s, first*n + c) {
            return false;
        }
    }
    true
}

fn order_relations_recursive(n: usize, prefix: Bitset, remainder: Bitset) -> usize {
    let mut current_remainder = remainder;
    let mut recursive_remainder = remainder;
    let mut count = 0;
    while current_remainder != 0 {
        let index = bitset_first_index_set(current_remainder);

        if is_transitive_fast(n, prefix, index) {
            let mut current = prefix;
            debug_assert!(!bitset_has(current, index));
            bitset_set(&mut current, index);

            debug_assert!(is_antisymmetric(n, current));
            debug_assert!(is_transitive(n, current));

            count += 1;
            bitset_clear(&mut recursive_remainder, index);

            let (a, b) = index_to_ab(n, index);
            let ba_index = b*n + a;
            let mut next_remainder = recursive_remainder;
            bitset_clear(&mut next_remainder, ba_index);
            count += order_relations_recursive(n, current, next_remainder);
        }

        bitset_clear(&mut current_remainder, index);
    }
    count
}

fn not_reflexive(n: usize) -> Bitset {
    let mut s = 0 as Bitset;
    for a in 0..n {
        for b in 0..n {
            if a == b {
                continue;
            }
            bitset_set(&mut s, a*n + b);
        }
    }
    assert_eq!(s.count_ones() as usize, n*n - n);
    s
}

fn order_relations(n: usize) -> usize {
    assert!(n <= 11);
    1 + order_relations_recursive(n, 0, not_reflexive(n))
}

fn main() {
    println!("{:?}", order_relations(8));
}
