use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut common_multiples: HashSet<u32> = HashSet::new();
    common_multiples.insert(0);

    for &m in factors {
        if m == 0 {
            continue;
        }

        for x in (m..limit).step_by(m as usize) {
            common_multiples.insert(x);
        }
    }

    common_multiples.iter().sum::<u32>()
}
