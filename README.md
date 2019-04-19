---
description: Documentation the primes-cli, and related binaries
---

# About Primes Cli

Primes CLI exposes a family of binaries including primes, primes-near, primes-between, primes-nth, and primes-which. It's all about the modest task of finding prime numbers in a certain range. It is simply sugar around the [primes crate](https://crates.io/crates/primes), to make it easier to invoke in a command-line context. The `primes` crate itself uses [Sieve Of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) as it's primary algorithm

primes is a family of commands that can be used to find prime numbers near a specified number.

## Value Proposition

It is occassionally useful to have a quick way to find a prime number near a particular value. I personally like to use prime numbers in cron-jobs, systemd timers, timeout settings, cache TTLs, and anywhere where there might be a risk of [Thundering Herd Problem](https://en.wikipedia.org/wiki/Thundering_herd_problem). Aside from that, I just like prime numbers. Aside from _that_, I needed an excuse to learn [Rust](https://www.rust-lang.org/) and the [crate ecosystem](https://doc.rust-lang.org/cargo/guide/).

## Primes

The command `primes` by itself does nothing useful, except produce the help screen, just like calling git or docker by itself would do.

## Primes Near

`primes-near`, or `primes near`takes one number as input and returns the nearest prime lower than that number, and the nearest prime higher. If the number you give it is itself prime, it returns 3 numbers

```text
primes-near 25            # returns 23,27
echo 100 | primes-near    # returns 97,101
primes near 13            # returns 11,13,17
primes near banana        # exits with error: not a number
```

## Primes Between

Takes two numbers \(unsigned integers\) and returns all the primes in that range, _inclusive_ or the lower-bound and upper-bound, so:

```text
primes between 17,29        # returns 17,23,29
primes between 17..29       # returns 17,23,29
primes-between 29 17        # returns 17,23,29 
```

## Primes Which

The set of prime numbers is an ordered and infinite list. 2 is the first prime, 3 is the second, which prime is 17?

```text
primes which 17                # returns 6
primes which 16                # exits with error: not a prime
```

## Primes n

What is the nth prime?

```text
primes n 6                    # returns 17
```

