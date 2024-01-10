use app_cmd::app_cmd;
use run_runners::run;

mod app_cmd;
mod config;
mod error;
mod exec;
mod run_runners;
mod utils;

pub use self::error::{Error, Result};

pub use app_cmd::VERSION;

fn main() {
	match cmd_run() {
		Ok(_) => (),
		Err(err) => println!("FAIL - {err}"),
	}
}

fn cmd_run() -> Result<()> {
	let app = app_cmd().get_matches();
	let input = app.get_one::<String>("INPUT");

	if let Some(input) = input {
		run(input)?;
	} else {
		// needs cmd_app version as the orginal got consumed by get_matches
		app_cmd().print_long_help()?;
		println!("\n");
	}

	Ok(())
}
