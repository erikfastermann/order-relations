fn bitset_has(s: u64, index: usize) -> bool {
    (s & (1 << index)) != 0
}

fn is_antisymmetric(n: usize, s: u64) -> bool {
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

fn is_transitive(n: usize, s: u64) -> bool {
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

fn order_relations(n: usize) -> usize {
    assert!(n <= 7);
    let size_cartesian_product = n*n;
    let reflexiv = (0..n).map(|i| i*n + i)
        .fold(0u64, |acc, i| acc | (1 << i));
    assert_eq!(reflexiv.count_ones(), n.try_into().unwrap());

    // Powerset has 2**n entries
    let max = 1u64 << (size_cartesian_product as u64);
    let mut count = 0;
    for i in 0..max {
        // Check is reflexiv
        if i&reflexiv != reflexiv {
            continue;
        }
        if is_antisymmetric(n, i) && is_transitive(n, i) {
            count += 1;
        }
    }
    return count;
}

fn main() {
    println!("{:?}", order_relations(5));
}
