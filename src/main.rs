//! # Primes CLI
//!
//! `primes-cli` is a small utility that helps you find small prime numbers

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

mod between;
mod die;
mod help;
mod near;

fn is_int(val: String) -> Result<(), String> {
	let _val: u64 = val.trim().parse().expect("The value must be an integer");
	return Ok(());
}

fn main() {
	let raw_args = App::new("primes")
		.about("a small utility that helps you find small prime numbers")
		.version("0.0.1")
		.author("Sean Macdonald <code_monk@fukt.ca>")
		.after_help(
			"Longer explanation to appear after the options when \
			 displaying the help information from --help or -h",
		)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(
			SubCommand::with_name("near")
				.about("find nearest lesser and nearest greater prime")
				.setting(AppSettings::ArgRequiredElseHelp)
				.arg(
					Arg::with_name("n")
						.help("an unsigned integer")
						.required(true)
						.validator(is_int),
				),
		)
		.subcommand(
			SubCommand::with_name("between")
				.about("find primes between n and m")
				.setting(AppSettings::ArgRequiredElseHelp)
				.arg(
					Arg::with_name("m")
						.help("lower bound")
						.required(true)
						.validator(is_int),
				)
				.arg(
					Arg::with_name("n")
						.help("upper bound")
						.required(true)
						.validator(is_int),
				),
		);
	let matches = &raw_args.get_matches();

	enum Invocation {
		Near(u64),
		Between(u64, u64),
		Help,
		Die(String),
	}

	fn route(cmd: Invocation) {
		match cmd {
			Invocation::Between(m, n) => between::main(m, n),
			Invocation::Near(n) => near::main(n),
			Invocation::Help => help::main(),
			Invocation::Die(msg) => die::main(msg),
		}
	}

	let invok = match matches.subcommand_name() {
		Some("near") => {
			let subcommand_params = matches.subcommand_matches("near").unwrap();
			match subcommand_params.value_of("n").unwrap().parse::<u64>() {
				Ok(n) => Invocation::Near(n),
				Err(_) => Invocation::Die(String::from("Unable to parse integer for 'near'")),
			}
		}
		Some("between") => {
			if let Some(submatches) = matches.subcommand_matches("between") {
				let n: u64 = submatches.value_of("n").unwrap().parse().unwrap();
				let m: u64 = submatches.value_of("m").unwrap().parse().unwrap();
				Invocation::Between(m, n)
			} else {
				Invocation::Die(String::from("could not get submatches for 'between'"))
			}
		}
		None => {
			println!("No subcommand was used");
			Invocation::Help
		}
		_ => {
			println!("Some other subcommand was used");
			Invocation::Help
		}
	};

	route(invok);

	//println!("{:?}", matches);
}
