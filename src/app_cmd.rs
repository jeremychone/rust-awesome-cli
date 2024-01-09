use clap::{crate_version, Command};

pub const VERSION: &str = crate_version!();

pub fn app_cmd() -> Command {
	Command::new("awesome-app")
		.version(VERSION)
		.about("Awesome CLI Runner")
		.subcommand(sub_dev())
}

fn sub_dev() -> Command {
	Command::new("dev").about("Starts hot-reload developement")
}
