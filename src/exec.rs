use crate::{Error, Result};
use std::io::{self, stdin, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use tokio::process::{Child as TokioChild, Command as TokioCommand};

#[allow(unused)]
pub fn prompt(message: &str, default: Option<&str>) -> Result<String> {
	print!("{message}");
	let _ = io::stdout().flush();

	let mut buf = String::new();
	stdin().read_line(&mut buf).map_err(|_| Error::StdinFailToReadLine)?;

	let val = buf.trim();

	let val = match (val.is_empty(), default) {
		(true, Some(default)) => default,
		(false, _) => val,
		(true, None) => val, // return the empty string (TODO: might want to return error)
	};

	Ok(val.to_string())
}

pub fn spawn_and_wait(cwd: Option<&Path>, cmd_str: &str, args: &[&str], print_exec: bool) -> Result<()> {
	let mut cmd = build_cmd(cwd, cmd_str, args);

	if print_exec {
		println!("> executing: {} {}", cmd_str, args.join(" "));
	}

	match cmd.spawn()?.wait() {
		Ok(status) => {
			if !status.success() {
				Err((cmd_str, args, status).into())
			} else {
				Ok(())
			}
		}
		Err(ex) => Err(ex.into()),
	}
}

pub fn spawn_tokio(cwd: Option<&Path>, cmd_str: &str, args: &[&str], print_exec: bool) -> Result<TokioChild> {
	if print_exec {
		println!("> executing: {} {}", cmd_str, args.join(" "));
	}
	let mut cmd = build_tokio_cmd(cwd, cmd_str, args);

	let child = cmd.spawn()?;

	Ok(child)
}

pub fn build_cmd(cwd: Option<&Path>, cmd: &str, args: &[&str]) -> Command {
	let mut cmd = Command::new(cmd);
	if let Some(cwd) = cwd {
		cmd.current_dir(cwd);
	}
	cmd.args(args);
	cmd
}

pub fn build_tokio_cmd(cwd: Option<&Path>, cmd: &str, args: &[&str]) -> TokioCommand {
	let mut cmd = TokioCommand::new(cmd);
	if let Some(cwd) = cwd {
		cmd.current_dir(cwd);
	}
	cmd.args(args);
	cmd
}

#[allow(unused)]
pub fn spawn_output(cwd: Option<&Path>, cmd_str: &str, args: &[&str], print_exec: bool) -> Result<String> {
	if print_exec {
		println!("> executing: {} {}", cmd_str, args.join(" "));
	}
	let mut cmd = build_cmd(cwd, cmd_str, args);

	match cmd.stdout(Stdio::piped()).output() {
		Err(ex) => Err(ex.into()),
		Ok(output) => {
			let txt = if output.status.success() {
				String::from_utf8(output.stdout)
			} else {
				String::from_utf8(output.stderr)
			};

			match txt {
				Err(ex) => Err(Error::Exec(cmd_str.to_string(), format!("{ex:?}"))),
				Ok(txt) => Ok(txt),
			}
		}
	}
}
