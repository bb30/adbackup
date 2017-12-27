use failure::Error;

use adb_command::AdbCommand;

#[derive(Debug, PartialEq, Clone)]
pub struct Restore {}

impl Restore {
    pub fn restore(device_id: Option<String>) -> Result<(), Error> {
        AdbCommand::command("restore".to_string())
            .with_args(vec!["backup.ab".to_string()])
            .with_device_id(device_id)
            .execute()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use devices::Device;
    use restore::Restore;

    //#[test]
    fn test_simple_restore() {
        if Device::list_devices().unwrap().len() > 0 {
            assert!(Restore::restore(Some("emulator-5554".to_string())).is_ok())
        }
    }
}
