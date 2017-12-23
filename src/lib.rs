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
        let mut device_list = String::from("Found the following devices:\r\n");
        devices.into_iter().for_each(|device|
            device_list = format!("{}\r\nId: '{}', Name: '{}'", device_list, device.id, device.name));
        Ok(device_list)
    } else {
        Ok(String::from("No device found. Make sure that you connect at least one device with enabled \
        debug options."))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
