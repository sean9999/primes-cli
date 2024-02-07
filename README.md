# About Primes Cli

Primes CLI is a binary called `primes`, exposing a family of subcommands including `primes near`, `primes between`, which are all about finding small prime numbers in a certain range (0 to 18,446,744,073,709,551,615 on 64 bit machines, o to 4,294,967,295 on 32 bit). The basic mechanics leverage [Sieve Of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) and [Prime Number Theorem](https://en.wikipedia.org/wiki/Prime_number_theorem), with optimisations to make it fast.

## Value Proposition

It is occassionally useful to have a quick way to find a prime number near a particular value. I personally like to use prime numbers in cron-jobs, systemd timers, timeout settings, cache TTLs, and anywhere where there might be a risk of [Thundering Herd Problem](https://en.wikipedia.org/wiki/Thundering_herd_problem). It was mainly built for fun. 

## Primes

The command `primes` by itself does nothing useful, except produce the help screen (equivilant to `primes help`).

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

## Primes Beneath

Goes all the way to the bottom, so:

```bash
$ primes beneath 25     # returns 2,3,5,7,11,13,17,19,23
```

## Primes Is

Basic primality test:

```bash
$ primes is 50     # returns no
$ primes is 53     # returns yes
```
