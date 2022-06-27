---
description: primes-cli is a small utility that helps you find prime numbers.
---

# About Primes Cli

Primes CLI exposes a family of subcommands including `primes near`, `primes between`, which are all about finding small (between 2 and 4,294,967,295) prime numbers in a certain range. It is simply sugar around the [primes crate](https://crates.io/crates/primes), to turn it into a command-line tool. The `primes` crate itself uses the [Sieve Of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) algorithm for heavy lifting.

## Value Proposition

It is occassionally useful to have a quick way to find a prime number near a particular value. I personally like to use prime numbers in cron-jobs, systemd timers, timeout settings, cache TTLs, and anywhere where there might be a risk of [Thundering Herd Problem](https://en.wikipedia.org/wiki/Thundering_herd_problem). Aside from that, I just like prime numbers. Aside from _that_, I needed an excuse to learn [Rust](https://www.rust-lang.org/) and the [crate ecosystem](https://doc.rust-lang.org/cargo/guide/).

## Primes

The command `primes` by itself does nothing useful, except produce the help screen (equivilant to `primes help`).

## Primes Help

`primes help` shows you the help screen. A good place to start.

## Primes Near

`primes near` takes one number as input and returns the nearest prime lower than that number, and the nearest prime higher. If the number you give it is itself prime, it returns 3 numbers

```bash
$ primes near 25            # returns 23,27
$ primes near 13            # returns 11,13,17
$ primes near banana        # exits with error: not a number
```

## Primes Between

Takes two numbers \(unsigned integers\) and returns all the primes in that range, _inclusive_, so:

```bash
$ primes between 17 29      # returns 17,23,29
```


