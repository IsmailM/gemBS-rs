use clap::ArgMatches;
use crate::cli::utils;

pub fn index_command(m: &ArgMatches) -> Result<(), &'static str> {
	let threads: Option<i32> = utils::from_arg_matches(m, "threads");
	println!("threads: {:?}", threads);
	Ok(())
}
