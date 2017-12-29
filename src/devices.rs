use failure::Error;

use adb_command::AdbCommand;

#[derive(Debug, PartialEq, Clone)]
pub struct Device {
    pub id: String,
    pub details: String,
}

impl Device {
    pub fn list_devices() -> Result<Vec<Device>, Error> {
        let output = AdbCommand::command("devices".to_string())
            .with_arg("-l".to_string())
            .execute()?;

        return Ok(Device::parse_devices(output));
    }

    fn parse_devices(unparsed_devices: String) -> Vec<Device> {
        let mut devices: Vec<Device> = Vec::new();

        unparsed_devices
            .lines()
            .filter(|x| !x.contains("List of devices attached"))
            .for_each(|unparsed_device| {
                let splitted_device = unparsed_device.split("device ").collect::<Vec<&str>>();

                if let Some(device_id) = splitted_device.get(0) {
                    if let Some(device_details) = splitted_device.get(1) {
                        let device = Device {
                            id: device_id.trim().to_string(),
                            details: device_details.trim().to_string(),
                        };
                        devices.push(device);
                    }
                }
            });
        devices
    }
}

#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use devices::Device;

    #[test]
    fn test_parse_mocked_devices() {
        let devices = vec![
            Device {
                id: "emulator-5554".to_string(),
                details: "product:sdk_google_phone_x86 model:Android_SDK_built_for_x86 \
                          device:generic_x86 transport_id:9"
                    .to_string(),
            },
            Device {
                id: "192.168.2.100:5555".to_string(),
                details:
                "product:lineage_oneplus3 model:ONEPLUS_A3003 device:OnePlus3T transport_id:8"
                    .to_string(),
            },
        ];

        let mocked_output = "List of devices attached\nemulator-5554          \
            device product:sdk_google_phone_x86 model:Android_SDK_built_for_x86 device:generic_x86 \
            transport_id:9 \n192.168.2.100:5555     device product:lineage_oneplus3 model:\
            ONEPLUS_A3003 device:OnePlus3T transport_id:8\n\n".to_string();
        assert_that!(Device::parse_devices(mocked_output), is(equal_to(devices)));
    }

    #[test]
    fn test_parse_mocked_no_connected_device() {
        let mocked_output = "List of devices attached\n\n\n".to_string();
        assert_that!(
            Device::parse_devices(mocked_output),
            is(equal_to(Vec::new()))
        );
    }
}
