pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = vec![];

    let mut value = n;

    for i in 2..=n {
        while value % i == 0 {
            primes.push(i);
            value /= i;
        }

        if value == 1 {
            break;
        }
    }

    primes
}
