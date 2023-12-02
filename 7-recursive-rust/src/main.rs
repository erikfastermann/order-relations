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

// TODO: use this
fn ab_to_index(n: usize, a: usize, b: usize) -> usize {
    a*n + b
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

fn order_relations_recursive(n: usize, prefix: Bitset, remainder: Bitset) -> usize {
    let mut recursive_remainder = remainder;
    let mut count = 0;
    // Might be faster to iterate over the set bits by checking trailing zeros or smth
    for index in 0..n*n {
        if !bitset_has(remainder, index) {
            continue;
        }

        let mut current = prefix;
        debug_assert!(!bitset_has(current, index));
        bitset_set(&mut current, index);

        if is_transitive(n, current) {
            debug_assert!(is_antisymmetric(n, current));

            count += 1;
            bitset_clear(&mut recursive_remainder, index);

            let (a, b) = index_to_ab(n, index);
            let ba_index = b*n + a;
            let mut next_remainder = recursive_remainder;
            bitset_clear(&mut next_remainder, ba_index);
            count += order_relations_recursive(n, current, next_remainder);
        }
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
    println!("{:?}", order_relations(9));
}
