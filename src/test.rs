#[cfg(test)]
mod tests {

    #[test]
    fn test_near_with_non_prime() {
        let want: Vec<usize> = vec![2999 as usize, 3001 as usize];
        let got: Vec<usize> = crate::primes::near(3000 as usize);
        assert!(want == got)
    }

    #[test]
    fn test_near_with_prime() {
        let want: Vec<usize> = vec![2999 as usize, 3001 as usize, 3011 as usize];
        let got: Vec<usize> = crate::primes::near(3001 as usize);
        assert!(want == got)
    }

    #[test]
    fn test_between_50_60() {
        let want: Vec<usize> = vec![53 as usize, 59 as usize];
        let got: Vec<usize> = crate::primes::between(50 as usize, 60 as usize);
        assert!(want == got)
    }

    #[test]
    fn test_between_two_primes() {
        let want: Vec<usize> = vec![11 as usize, 13 as usize, 17 as usize];
        let got: Vec<usize> = crate::primes::between(11 as usize, 17 as usize);
        assert!(want == got)
    }
}
