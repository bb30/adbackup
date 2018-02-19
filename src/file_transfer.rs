use failure::Error;

use adb_command::AdbCommand;

#[derive(Debug, PartialEq, Clone)]
pub struct FileTransfer {}

impl FileTransfer {
    pub fn pull(device_id: Option<&str>, path: &str) -> Result<(), Error> {
        AdbCommand::command("pull")
            .with_args(vec![path, "-a"])
            .with_device_id(device_id)
            .execute()?;

        Ok(())
    }

    pub fn push(
        device_id: Option<&str>,
        src_path: &str,
        dst_path: &str,
    ) -> Result<(), Error> {
        AdbCommand::command("push")
            .with_args(vec![src_path, dst_path])
            .with_device_id(device_id)
            .execute()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use devices::Device;
    use file_transfer::FileTransfer;

    //#[test]
    #[allow(dead_code)]
    fn test_simple_pull() {
        if Device::list_devices().unwrap().len() > 0 {
            assert!(
                FileTransfer::pull(Some("emulator-5554"), "/sdcard/la/")
                    .is_ok()
            )
        }
    }

    //#[test]
    #[allow(dead_code)]
    fn test_simple_push() {
        if Device::list_devices().unwrap().len() > 0 {
            assert!(
                FileTransfer::push(
                    Some("emulator-5554"),
                    "Cargo.toml",
                    "/sdcard/la/",
                ).is_ok()
            )
        }
    }
}
