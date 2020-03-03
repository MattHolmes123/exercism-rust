pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut ct: u32 = 0;

    for i in (3..).step_by(2) {
        if is_prime(i) {
            ct += 1;

            if ct == n {
                return i;
            }
        }
    }
    unreachable!();
}

fn is_prime(n: u32) -> bool {
    let sqrt_n: u32 = (n as f32).sqrt() as u32;

    for div in 2..=sqrt_n {
        if n % div == 0 {
            return false;
        }
    }
    true
}
