use primes::PrimeSet;

fn vector_as_list(mut vec: Vec<u64>) -> String {
    vec.reverse();
    let mut s = String::from("");
    if let Some(first) = vec.pop() {
        s.push_str(&first.to_string());
        while let Some(top) = vec.pop() {
            s.push_str(",");
            s.push_str(&top.to_string());
        }
    } else {
        s.push_str("Could not pop first element off vector");
    }
    s
}

pub fn main(lower_bound: u64, upper_bound: u64) {
    let mut pset = PrimeSet::new();
    let mut r: Vec<u64> = Vec::new();
    let (low_index, lowest_prime) = pset.find(lower_bound);
    let mut i = low_index;
    let mut p = lowest_prime;
    while p <= upper_bound {
        r.push(p);
        i = i + 1;
        p = pset.get(i);
    }
    //r.reverse();
    println!("{}", vector_as_list(r));
}
