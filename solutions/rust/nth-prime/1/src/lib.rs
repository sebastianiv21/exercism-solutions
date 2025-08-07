pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2; // First prime is 2
    }

    let mut count = 1; // We already have the first prime (2)
    let mut candidate = 3; // Start checking from 3

    while count <= n {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 2; // Only check odd numbers after 2
    }

    candidate - 2 // This should never be reached
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Check odd divisors from 3 up to sqrt(n)
    let limit = (n as f64).sqrt() as u32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}
