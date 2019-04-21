use primes::PrimeSet;

pub fn main(n: u64) {
    let mut pset = PrimeSet::new();

    let (p2i, p2) = pset.find(n);
    let p1 = pset.get(p2i - 1);
    if p2 == n {
        let p3 = pset.get(p2i + 1);
        println!("{},{},{}", p1, p2, p3);
    } else {
        println!("{},{}", p1, p2);
    }
}
