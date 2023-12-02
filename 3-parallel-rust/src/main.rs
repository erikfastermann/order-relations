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

const CORE_COUNT: usize = 4;

fn order_relations_parallel(n: usize) -> usize {
    assert!(n <= 7);
    let size_cartesian_product = n*n;
    let reflexiv = (0..n).map(|i| i*n + i)
        .fold(0u64, |acc, i| acc | (1 << i));
    assert_eq!(reflexiv.count_ones(), n.try_into().unwrap());

    // Powerset has 2**n entries
    let max = 1u64 << (size_cartesian_product as u64);
    let per_core = max / (CORE_COUNT as u64);
    let mut current_max = 0;
    
    let mut handles = Vec::new();
    for _ in 0..CORE_COUNT-1 {
        let handle = std::thread::spawn(move || count_segment(n, reflexiv, current_max, current_max + per_core));
        handles.push(handle);
        current_max += per_core;
    }

    let mut count = count_segment(n, reflexiv, current_max, max);
    for handle in handles {
        count += handle.join().unwrap();
    }
    return count;
}

fn count_segment(n: usize, reflexiv: u64, from_inclusive: u64, to_exclusive: u64) -> usize {
    let mut count = 0;
    for i in from_inclusive..to_exclusive {
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
    println!("{:?}", order_relations_parallel(6));
}
