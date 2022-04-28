use twilight_bot_utils::prelude::*;

use std::fmt;

#[derive(Debug)]
pub(super) enum DatabaseConnectionError {
	DbScriptNotFound,
	ChildProcessNoStdout,
	UnexpectedStdoutEnd
}

impl fmt::Display for DatabaseConnectionError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use DatabaseConnectionError::*;

		match self {
			DbScriptNotFound => {
				write!(f, "db.mjs not found")
			}
			ChildProcessNoStdout => {
				write!(f, "db child process did not have an stdout")
			}
			UnexpectedStdoutEnd => {
				write!(f, "unexpected of child stdout")
			}
		}
	}
}

impl Error for DatabaseConnectionError {}
