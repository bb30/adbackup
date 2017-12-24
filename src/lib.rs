mod devices;
mod logging;

extern crate fern;
extern crate chrono;

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

extern crate failure;
use failure::Error;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn setup_logging(verbosity: u64) {
    logging::setup_logging(verbosity)
        .expect("failed to initialize logging.");
}

pub fn get_printable_device_list() -> Result<String, Error> {
    use devices::Device;
    let devices = Device::list_devices()?;

    if devices.len() > 0 {
        let devices_found = "Found the following devices:";
        info!("{}", devices_found);

        let mut device_list = format!("{}\r\n", devices_found);

        devices.into_iter().for_each(|device| {
            let device_info = format!("Id: '{}', Name: '{}'", device.id, device.name);
            info!("{}", device_info);
            device_list = format!("{}\r\n{}", device_list, device_info)
        });

        Ok(device_list)
    } else {
        let no_devices_found = "No device found. Make sure that you connect at least one device with enabled \
        debug options.";
        warn!("{}", no_devices_found);

        Ok(String::from(no_devices_found))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
