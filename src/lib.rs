mod devices;
mod logging;
mod backup;
mod restore;
mod adb_command;
mod file_transfer;

extern crate chrono;
extern crate fern;

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

extern crate failure;

use failure::{err_msg, Error};

use backup::Backup;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn setup_logging(verbosity: u64) {
    logging::setup_logging(verbosity).expect("failed to initialize logging.");
}

pub fn get_printable_device_list() -> Result<String, Error> {
    use devices::Device;
    let devices = Device::list_devices()?;

    if devices.len() > 0 {
        let devices_found = "Found the following devices:";
        info!("{}", devices_found);

        let mut device_list = format!("{}\r\n", devices_found);

        devices.into_iter().for_each(|device| {
            let device_info = format!("Id: '{}', details: '{}'", device.id, device.details);
            info!("{}", device_info);
            device_list = format!("{}\r\n{}", device_list, device_info)
        });

        Ok(device_list)
    } else {
        let no_devices_found =
            "No device found. Make sure that you connect at least one device with enabled \
             debug options.";
        warn!("{}", no_devices_found);

        Ok(String::from(no_devices_found))
    }
}

pub fn get_printable_app_list(device_id: Option<&str>) -> Result<String, Error> {
    check_too_much_devices(&device_id)?;

    let apps = Backup::list_apps(device_id)?;

    if apps.len() > 0 {
        let app_found = "Found the following app(s) on device:";
        info!("{}", app_found);

        let mut app_list = format!("{}\r\n", app_found);

        apps.into_iter().for_each(|app| {
            let app_name = format!("{}\n", app);
            info!("{}", app_name);
            app_list = format!("{}{}", app_list, app_name)
        });

        Ok(app_list)
    } else {
        let no_apps_found = "No packages found.";
        warn!("{}", no_apps_found);

        Ok(String::from(no_apps_found))
    }
}

pub fn backup(
    device_id: Option<&str>,
    apk: Option<&str>,
    shared: Option<&str>,
    system: Option<&str>,
    only_specified: Option<&str>,
) -> Result<String, Error> {
    check_too_much_devices(&device_id)?;

    let mut backup_options = backup::BackupOptions::default();

    if let Some(device_id) = device_id {
        backup_options = backup_options.with_device_id(device_id);
    }
    if let Some(_) = apk {
        backup_options = backup_options.with_applications();
    }
    if let Some(_) = shared {
        backup_options = backup_options.with_shared_storage();
    }
    if let Some(_) = system {
        backup_options = backup_options.with_system_apps();
    }
    if let Some(only_specified) = only_specified {
        backup_options = backup_options.with_only_specified_app(only_specified);
    }

    Backup::backup(backup_options)?;

    let backup_finished = "Backup finished.";
    info!("{}", backup_finished);

    Ok(String::from(backup_finished))
}

fn check_too_much_devices(device_id: &Option<&str>) -> Result<(), Error> {
    if device_id.is_none() && devices::Device::list_devices()?.len() > 1 {
        let error =
            "More than one device connected and no device provided.\n \
             Please execute adbackup again, with `--device` and one of the following device ids:\n";

        let error_message = format!("{}\n{}", error, get_printable_device_list()?);
        info!("{}", error_message);
        return Err(err_msg(error_message));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
