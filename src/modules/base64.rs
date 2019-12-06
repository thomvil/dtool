use clap::{SubCommand, Arg, ArgMatches};
use crate::modules::Command;
use std::io;
use std::io::BufRead;
use hex;
use base64;

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
	vec![
		Command {
			app: SubCommand::with_name("h2b64").about("Convert hex to base64").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: h2b64,
		},
		Command {
			app: SubCommand::with_name("b642h").about("Convert base64 to hex").arg(
				Arg::with_name("INPUT")
					.required(false)
					.index(1)),
			f: b642h,
		},
	]
}

fn h2b64(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let input = input.trim_start_matches("0x");

	let input = hex::decode(input).map_err(|_| "Convert failed")?;

	let result = base64::encode(&input);

	Ok(vec![result])
}

fn b642h(matches: &ArgMatches) -> Result<Vec<String>, String> {

	let input = match matches.value_of("INPUT") {
		Some(input) => input.to_string(),
		None => io::stdin().lock().lines().map(|l|l.unwrap()).collect::<Vec<String>>().join(""),
	};

	let result = base64::decode(&input).map_err(|_| "Convert failed")?;
	let result = hex::encode(result);
	let result = format!("0x{}", result);

	Ok(vec![result])
}