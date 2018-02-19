use failure::Error;

use adb_command::AdbCommand;

#[derive(Debug, PartialEq, Clone)]
pub struct Restore {}

impl Restore {
    pub fn restore(device_id: &str) -> Result<(), Error> {
        AdbCommand::command("restore")
            .with_arg(&format!("{}.ab", device_id))
            .with_device_id(Some(device_id))
            .execute()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use devices::Device;
    use restore::Restore;

    //#[test]
    #[allow(dead_code)]
    fn test_simple_restore() {
        if Device::list_devices().unwrap().len() > 0 {
            assert!(Restore::restore("emulator-5554").is_ok())
        }
    }
}
