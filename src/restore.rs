use std::process::{Command, Stdio};

use failure::{Error, err_msg};


#[derive(Debug, PartialEq, Clone)]
pub struct Restore {}

impl Restore {
    pub fn restore(device_id: Option<String>) -> Result<(), Error> {
        let mut child = Command::new("adb");

        if let Some(device_id) = device_id {
            child.arg("-s")
                .arg(device_id);
        }

        child.arg("restore")
            .arg("backup.ab");

        let output = child
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if output.status.success() {
            let output_message = String::from_utf8(output.stdout)?;
            trace!("output message from command: {}", output_message);
            Ok(())
        } else {
            let error_output = String::from_utf8(output.stderr)?;
            return Err(err_msg(format!("Error executing restore {}", error_output)));
        }
    }
}


#[cfg(test)]
mod tests {
    use devices::Device;
    use restore::Restore;

    #[test]
    fn test_simple_restore() {
        if Device::list_devices().unwrap().len() > 0 {
            assert!(Restore::restore(Some("emulator-5554".to_string())).is_ok())
        }
    }
}
