/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use std::{
	ffi::OsStr,
	process::Stdio,
};
use tokio::process::Command;
use thiserror::Error;

/// Internal errors in the VS Code CLI.
/// Note: other error should be migrated to this type gradually
#[derive(Error, Debug)]
pub enum CodeError {
	#[error("platform not currently supported: {0}")]
	UnsupportedPlatform(String),
	#[error("This machine does not meet {name}'s prerequisites, expected either...: {bullets}")]
	PrerequisitesFailed { name: &'static str, bullets: String },
	#[error("failed to run command \"{command}\" (code {code}): {output}")]
	CommandFailed {
		command: String,
		code: i32,
		output: String,
	},
 }


pub async fn capture_command<A, I, S>(
	command_str: A,
	args: I,
) -> Result<std::process::Output, CodeError>
where
	A: AsRef<OsStr>,
	I: IntoIterator<Item = S>,
	S: AsRef<OsStr>,
{
	new_tokio_command(&command_str)
		.args(args)
		.stdin(Stdio::null())
		.stdout(Stdio::piped())
		.output()
		.await
		.map_err(|e| CodeError::CommandFailed {
			command: command_str.as_ref().to_string_lossy().to_string(),
			code: -1,
			output: e.to_string(),
		})
}

/// Makes a new Command, setting flags to avoid extra windows on win32
#[cfg(windows)]
pub fn new_tokio_command(exe: impl AsRef<OsStr>) -> Command {
	let mut p = tokio::process::Command::new(exe);
	p.creation_flags(winapi::um::winbase::CREATE_NO_WINDOW);
	p
}

/// Makes a new Command, setting flags to avoid extra windows on win32
#[cfg(not(windows))]
pub fn new_tokio_command(exe: impl AsRef<OsStr>) -> Command {
	tokio::process::Command::new(exe)
}
