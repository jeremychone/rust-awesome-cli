use std::process::ExitStatus;
use toml::Value;

pub type Result<R> = std::result::Result<R, Error>;

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("group name or command name invalid. Does not contain at least one element. Was '{0}'")]
	RunRefNoParts(String),

	#[error("Fail to execute {0} cause: {1}")]
	Exec(String, String),

	#[error("'Awesome.toml' file not found. Should be added where 'awesome` command get called.")]
	AwesomTomlNotFound,

	#[error("Runner has no 'name' or 'ref' property. Value: {0:?}")]
	RunnerHasNoNameOrRef(Value),

	#[error("Do not support Runner ref=... to another Runner that is also a ref=... (for now). {0}")]
	DoNotSupportRefToRefYet(String),

	#[error("Path not safe to delete {0}")]
	PathNotSafeToDelete(String),

	#[error("Still have some unresolved fefed Runners. {0}")]
	StillHaveUnresolvedRefedRunners(Value),

	#[error("Directory {0} already exist. Cancelling.")]
	DirAlreadyExist(String),

	#[error("git command line not found. Required for awesome-app.")]
	GitNotPresent,

	#[error("Fail to parse Awesome.toml. Cause: {0}")]
	FailParsingConfig(toml::de::Error),

	#[error("Fail to parse runner. Cause: {0}")]
	FailParsingRunner(toml::de::Error),

	#[error("Solo runner '{0}' defined multiple time")]
	SoloRunnerMultipleDef(String),

	#[error("Awesome.toml does not seem to be valid.")]
	AwesomeTomlInvalid,

	#[error("Fail to read line")]
	StdinFailToReadLine,

	#[error(transparent)]
	IO(#[from] std::io::Error),
}

type ExecWithExitStatus<'a> = (&'a str, &'a [&'a str], ExitStatus);

impl<'a> From<ExecWithExitStatus<'a>> for Error {
	fn from(val: ExecWithExitStatus) -> Self {
		Error::Exec(val.0.to_string(), "".to_string())
	}
}
