//  return one prime beneath and one prime above the value given
//  if the value given is prime, return that too
pub fn near(n: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::with_capacity(3);
    //  low num
    if n > 3 {
        for i in (2..n).rev() {
            if is_prime(i) {
                primes.push(i);
                break;
            }
        }
    }
    //  mid num
    if is_prime(n) {
        primes.push(n);
    }
    //  high num
    let mut next_num: usize = n + 1;
    while !is_prime(next_num) {
        next_num += 1;
    }
    primes.push(next_num);
    return primes;
}

//  call beneath() and then slice off the early bits
pub fn between(a: usize, b: usize) -> Vec<usize> {
    let all_primes = beneath(b);
    let mut i = 0;
    for p in &all_primes {
        if p < &a {
            i += 1;
        }
    }
    let slice = &all_primes[i..];
    return slice.to_vec();
}

//  primality tester using a builder pattern, with cache as optimisation
pub fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    let s = (n as f64).sqrt().ceil() as usize;
    let mut cached_primes: Vec<usize> = Vec::with_capacity(n_primes_beneath(s));
    for i in 2..=s {
        if !is_multiple_of(i, &cached_primes) {
            cached_primes.push(i);
            if n > i && n % i == 0 {
                return false;
            }
        }
    }
    return true;
}

//  return all primes beneath the value provided
pub fn beneath(upper: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::with_capacity(n_primes_beneath(upper));
    for i in 2..=upper {
        if !is_multiple_of(i, &primes) {
            primes.push(i);
        }
    }
    return primes;
}

//  use prime number theorem to find approximate upper limit
fn n_primes_beneath(n: usize) -> usize {
    if n < 2 {
        return 0;
    }
    if n == 3 {
        return 1;
    }
    if n == 4 {
        return 2;
    }
    let n_float = n as f64;
    let approx_primes: f64 = n_float / n_float.ln() + 1 as f64;
    approx_primes.ceil() as usize
}

//  is n divisible by any of the primes in arr?
// fn is_divisible_by(n: usize, arr: &Vec<usize>) -> bool {
//     if arr.len() == 0 {
//         return false;
//     }
//     for m in arr {
//         if n > *m && n % *m == 0 {
//             return true;
//         }
//     }
//     return false;
// }

//  is n a multiple of any of the primes in arr?
fn is_multiple_of(n: usize, arr: &Vec<usize>) -> bool {
    if arr.len() == 0 {
        return false;
    }
    for m in arr {
        if n > *m && n % *m == 0 {
            return true;
        }
    }
    return false;
}