use std::process::{Command, Stdio};
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Device {
    pub id: String,
    pub name: String
}

impl Device {
    pub fn list_devices() -> Option<Vec<Device>> {
        let child = Command::new("adb")
            .arg("devices")
            .stdout(Stdio::piped())
            .spawn().unwrap(); // @SailReal FIXME with failure

        let output = child
            .wait_with_output().unwrap(); // @SailReal FIXME with failure

        if output.status.success() {
            let output_message = String::from_utf8(output.stdout).unwrap(); // @SailReal FIXME with failure
            trace!("output message from command: {}", output_message);
            return Device::parse_devices(output_message);
        }

        None
    }

    fn parse_devices(unparsed_devices: String) -> Option<Vec<Device>> {
        let mut devices: Vec<Device> = Vec::new();

        unparsed_devices
            .split('\n')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|x| !x.contains("List of devices attached"))
            .for_each(|unparsed_device| {
                let splitted_device = unparsed_device.split('\t').collect::<Vec<&str>>();

                if let Some(device_id) = splitted_device.get(0) {
                    if let Some(device_name) = splitted_device.get(1) {
                        let device = Device {
                            id: device_id.to_string(),
                            name: device_name.to_string()
                        };
                        devices.push(device);
                    }
                }
            });

        if devices.len() > 0 {
            return Some(devices);
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use devices;
    use hamcrest::prelude::*;
    use devices::Device;

    #[test]
    fn test_parse_mocked_devices() {
        let device = vec![Device {
            id: "5698fe75".to_string(),
            name: "device".to_string()
        }, Device {
            id: "emulator-5554".to_string(),
            name: "device".to_string()
        }];

        let mocked_output = "List of devices attached\n5698fe75\tdevice\nemulator-5554\tdevice\n\n\n".to_string();
        assert_that!(Device::parse_devices(mocked_output), is(equal_to(Some(device))));
    }

    #[test]
    fn test_parse_mocked_no_connected_device() {
        let mocked_output = "List of devices attached\n\n\n".to_string();
        assert_that!(Device::parse_devices(mocked_output), is(equal_to(None)));
    }
}
