use std::process::{Command, Stdio};

use failure::{Error, err_msg};

#[derive(Debug, PartialEq, Clone)]
pub struct AdbCommand {
    command: String,
    device_id: Option<String>,
    args: Vec<String>,
}

impl AdbCommand {
    pub fn command(command: String) -> Self {
        AdbCommand {
            command,
            device_id: None,
            args: Vec::new(),
        }
    }

    pub fn with_args(self, args: Vec<String>) -> Self {
        AdbCommand {
            args,
            ..self
        }
    }

    pub fn with_device_id(self, device_id: Option<String>) -> Self {
        AdbCommand {
            device_id,
            ..self
        }
    }

    pub fn execute(self) -> Result<String, Error> {
        let mut command = Command::new("adb");

        if let Some(device_id) = self.device_id {
            command.arg("-s")
                .arg(device_id);
        }

        command.arg(self.command);
        if self.args.len() > 0 {
            command.args(self.args);
        }

        let output = command
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if output.status.success() {
            let output_message = String::from_utf8(output.stdout)?;
            trace!("output message from command: {}", output_message);
            Ok(output_message)
        } else {
            let error_output = String::from_utf8(output.stderr)?;
            return Err(err_msg(format!("Error executing command {}", error_output)));
        }
    }
}