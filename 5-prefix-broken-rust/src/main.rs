use std::collections::HashSet;

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

    // Start with 1 for the base case with just reflexiv entries.
    // Otherwise reflexiv entries can be just ignored.
    let mut count = 1usize;
    let mut found = HashSet::from_iter([0u64]);
    let mut current = HashSet::new();
    let mut runs = 0;
    while found.len() > 0 {
        runs += 1;
        println!("{} {}", found.len(), count);
        for prefix in found.iter().copied() {
            for a in 0..n {
                for b in 0..n {
                    if a == b {
                        continue;
                    }

                    let index = a*n + b;
                    if bitset_has(prefix, index) {
                        continue;
                    }
                    let r = prefix | ((1 << index) as u64);
                    if current.contains(&r) {
                        continue;
                    }

                    if is_antisymmetric(n, r) && is_transitive(n, r) {
                        count += 1;
                        current.insert(r);
                    }
                }
            }
        }

        std::mem::swap(&mut found, &mut current);
        current.clear();
    }

    println!("{runs}");
    count
}

fn main() {
    println!("{:?}", order_relations(4));
}
