// #![allow(unused)]
use crate::prelude::*;
use app_cmd::app_cmd;
use run_dev::run_dev;

mod app_cmd;
mod config;
mod error;
mod exec;
mod prelude;
mod run_dev;
mod utils;

pub use app_cmd::VERSION;

fn main() {
	match cmd_run() {
		Ok(_) => (),
		Err(err) => println!("FAIL - {err}"),
	}
}

fn cmd_run() -> Result<()> {
	let app = app_cmd().get_matches();

	match app.subcommand() {
		Some(("dev", sub_cmd)) => run_dev(sub_cmd)?,
		_ => {
			// needs cmd_app version as the orginal got consumed by get_matches
			app_cmd().print_long_help()?;
			println!("\n");
		}
	}

	Ok(())
}
