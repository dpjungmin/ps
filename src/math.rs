/// Euclid's greatest common divisor (GCD) algorithm.
pub fn gcd(mut p: i64, mut q: i64) -> i64 {
    while q != 0 {
        p %= q;
        std::mem::swap(&mut p, &mut q);
    }

    p
}

/// Returns the least common multiple (LCM) of `p` and `q`.
pub fn lcm(p: i64, q: i64) -> i64 {
    p * q / gcd(p, q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1, 2), 1);
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(10, 19), 1);
        assert_eq!(gcd(10, 18), 2);
        assert_eq!(gcd(10, 15), 5);
        assert_eq!(gcd(10, 20), 10);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(1, 2), 2);
        assert_eq!(lcm(2, 2), 2);
        assert_eq!(lcm(10, 19), 190);
        assert_eq!(lcm(10, 18), 90);
        assert_eq!(lcm(10, 15), 30);
        assert_eq!(lcm(10, 20), 20);
    }
}
