use std::process::{Command, Stdio};

use failure::{err_msg, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct AdbCommand<'a> {
    command: &'a str,
    device_id: Option<&'a str>,
    args: Vec<&'a str>,
}

impl<'a> AdbCommand<'a> {
    pub fn command(command: &'a str) -> Self {
        AdbCommand {
            command,
            device_id: None,
            args: Vec::new(),
        }
    }

    pub fn with_args(self, args: Vec<&'a str>) -> Self {
        AdbCommand { args, ..self }
    }

    pub fn with_arg(self, arg: &'a str) -> Self {
        let mut args = self.args;
        args.push(arg);
        AdbCommand { args, ..self }
    }

    pub fn with_device_id(self, device_id: Option<&'a str>) -> Self {
        AdbCommand { device_id, ..self }
    }

    pub fn execute(self) -> Result<String, Error> {
        let mut command = Command::new("adb");

        if let Some(device_id) = self.device_id {
            command.arg("-s").arg(device_id);
        }

        command.arg(&self.command);

        if !self.args.is_empty() {
            command.args(self.args);
        }

        trace!("Executing command: {}", self.command);

        let output = command.stderr(Stdio::piped()).output()?;

        if output.status.success() {
            let output_message = String::from_utf8_lossy(&output.stdout);
            trace!("output message from {}: {}", self.command, output_message);
            Ok(output_message.to_string())
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            Err(err_msg(format!(
                "Error executing {}.\n {}",
                self.command, error_message
            )))
        }
    }
}
