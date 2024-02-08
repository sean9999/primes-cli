#![feature(test)]

//mod lib;
mod test;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "primes")]
#[command(about = "a utility for finding prime numbers", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// primes near 100 = 97 101
    #[command(arg_required_else_help = true)]
    Near { num: usize },
    /// primes between 90 100 = 91 93 97
    Between {
        #[arg(value_name = "LOW_NUM")]
        lownum: usize,
        #[arg(value_name = "HIGH_NUM")]
        highnum: usize,
    },
    /// primes is 100 = no
    #[command(arg_required_else_help = true)]
    Is { num: usize },
    /// primes beneath 10 = [2,3,5,7]
    Beneath { num: usize },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Near { num } => {
            let nums = primes::near(num);
            for num in &nums {
                println!("{}", num);
            }
        }
        Commands::Between { highnum, lownum } => {
            let nums = primes::between(lownum, highnum);
            for num in &nums {
                println!("{}", num);
            }
        }
        Commands::Is { num } => {
            let is = primes::is_prime(num);
            if is {
                println!("yes")
            } else {
                println!("no")
            }
        }
        Commands::Beneath { num } => {
            let nums = primes::beneath(num);
            for num in &nums {
                println!("{}", num);
            }
        }
    }
}
