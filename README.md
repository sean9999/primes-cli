---
description: primes-cli is a small utility that helps you find small prime numbers.
---

# About Primes Cli

Primes CLI exposes a family of subcommands including `primes near`, `primes between`, `primes nth`, and `primes which`, which are all about finding small (between 2 and 4,294,967,295) prime numbers in a certain range. It is simply sugar around the [primes crate](https://crates.io/crates/primes), to turn it into a command-line tool. The `primes` crate itself uses the [Sieve Of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) algorithm for heavy lifting.

## Value Proposition

It is occassionally useful to have a quick way to find a prime number near a particular value. I personally like to use prime numbers in cron-jobs, systemd timers, timeout settings, cache TTLs, and anywhere where there might be a risk of [Thundering Herd Problem](https://en.wikipedia.org/wiki/Thundering_herd_problem). Aside from that, I just like prime numbers. Aside from _that_, I needed an excuse to learn [Rust](https://www.rust-lang.org/) and the [crate ecosystem](https://doc.rust-lang.org/cargo/guide/).

## Primes

The command `primes` by itself does nothing useful, except produce the help screen (equivilant to `primes help`), just like calling git or docker by itself would do.

++ @Notice: In the future, calling `primes` will result in the app using heuristics to guess what you wanted to do. For now, it simply outputs help.

## Primes Help

`primes help` shows you the help screen. A good place to start.

## Primes Near

`primes near` takes one number as input and returns the nearest prime lower than that number, and the nearest prime higher. If the number you give it is itself prime, it returns 3 numbers

```bash
$ primes-near 25            # returns 23,27
$ echo 100 | primes-near    # returns 97,101
$ primes near 13            # returns 11,13,17
$ primes near banana        # exits with error: not a number
```

## Primes Between

Takes two numbers \(unsigned integers\) and returns all the primes in that range, _inclusive_ or the lower-bound and upper-bound, so:

```bash
$ primes between 17,29      # returns 17,23,29
$ primes between 17..29     # returns 17,23,29
$ primes between 29 17      # returns 17,23,29
```

## Primes Which

The set of prime numbers is an ordered and infinite list. 2 is the first prime, 3 is the second, which prime is 17?

```shell
$ primes which 17           # returns 6
$ primes which 16           # exits with error: not a prime
```

## Primes n

What is the nth prime?

```shell
$ primes n 6                # returns 17
```
