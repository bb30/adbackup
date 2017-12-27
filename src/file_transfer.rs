use failure::Error;

use adb_command::AdbCommand;

#[derive(Debug, PartialEq, Clone)]
pub struct FileTransfer {}

impl FileTransfer {
    pub fn pull(device_id: Option<String>, path: String) -> Result<(), Error> {
        AdbCommand::command("pull".to_string())
            .with_args(vec![path, "-a".to_string()])
            .with_device_id(device_id)
            .execute()?;

        Ok(())
    }

    pub fn push(
        device_id: Option<String>,
        src_path: String,
        dst_path: String,
    ) -> Result<(), Error> {
        AdbCommand::command("push".to_string())
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
    fn test_simple_pull() {
        if Device::list_devices().unwrap().len() > 0 {
            assert!(
                FileTransfer::pull(Some("emulator-5554".to_string()), "/sdcard/la/".to_string())
                    .is_ok()
            )
        }
    }

    //#[test]
    fn test_simple_push() {
        if Device::list_devices().unwrap().len() > 0 {
            assert!(
                FileTransfer::push(
                    Some("emulator-5554".to_string()),
                    "Cargo.toml".to_string(),
                    "/sdcard/la/".to_string()
                ).is_ok()
            )
        }
    }
}
