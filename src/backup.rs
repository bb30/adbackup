use std::process::{Command, Stdio};

use failure::{Error, err_msg};

#[derive(Debug, PartialEq, Clone)]
pub struct BackupOptions {
    device_id: Option<String>,
    applications: String,
    shared_storage: String,
    system_apps: String,
    only_specified_apps: String,
}

impl BackupOptions {
    pub fn default() -> Self {
        BackupOptions {
            device_id: None,
            applications: "-noapk".to_string(),
            shared_storage: "-noshared".to_string(),
            system_apps: "-nosystem".to_string(),
            only_specified_apps: "-all".to_string(),
        }
    }

    pub fn with_device_id(self, device_id: String) -> Self {
        BackupOptions {
            device_id: Some(device_id),
            ..self
        }
    }

    pub fn with_applications(self) -> Self {
        BackupOptions {
            applications: "-apk".to_string(),
            ..self
        }
    }

    pub fn with_shared_storage(self) -> Self {
        BackupOptions {
            shared_storage: "-shared".to_string(),
            ..self
        }
    }

    pub fn with_system_apps(self) -> Self {
        BackupOptions {
            system_apps: "-system".to_string(),
            ..self
        }
    }

    pub fn with_only_specified_apps(self, apps: String) -> Self {
        BackupOptions {
            only_specified_apps: apps,
            ..self
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Backup {}

impl Backup {
    pub fn backup(backup_options: BackupOptions) -> Result<(), Error> {
        let mut child = Command::new("adb");

        if let Some(device_id) = backup_options.device_id {
            child.arg("-s")
                .arg(device_id);
        }

        child.arg("backup")
            .arg(&backup_options.applications)
            .arg(&backup_options.shared_storage)
            .arg(&backup_options.system_apps)
            .arg(&backup_options.applications)
            .arg(&backup_options.only_specified_apps);


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
            return Err(err_msg(format!("Error executing backup {}", error_output)));
        }
    }

    pub fn list_apps(device_id: Option<String>) -> Result<Vec<String>, Error> {
        let mut child = Command::new("adb");

        if let Some(device_id) = device_id {
            child.arg("-s")
                .arg(device_id);
        }

        child.args(&["shell", "pm", "list", "packages"]);

        let output = child
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if output.status.success() {
            let output_message = String::from_utf8(output.stdout)?;
            trace!("output message from command: {}", output_message);
            return Ok(Backup::parse_list_apps(output_message));
        } else {
            let error_output = String::from_utf8(output.stderr)?;
            return Err(err_msg(format!("Error executing list apps {}", error_output)));
        }
    }

    fn parse_list_apps(command_response: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        command_response
            .split('\n')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|x| !x.contains("package:com.android.")
                && !x.contains("package:com.google.android."))
            .for_each(|package| {
                let splitted_package = package.split(':').collect::<Vec<&str>>();

                if let Some(package_name) = splitted_package.get(1) {
                    result.push(package_name.to_string());
                }
            });

        result
    }
}


#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use devices::Device;
    use backup::{BackupOptions, Backup};

    #[test]
    fn test_simple_backup() {
        if Device::list_devices().unwrap().len() > 0 {
            let options = BackupOptions::default()
                .with_device_name("emulator-5554".to_string())
                .with_applications()
                .with_shared_storage()
                .with_system_apps();

            Backup::backup(options).unwrap()
        }
    }

    #[test]
    fn test_parse_list_apps() {
        let mocked_output = "package:com.android.smoketest\n\
            package:com.android.cts.priv.ctsshim\npackage:org.cryptomator\n
            package:com.google.android.youtube\npackage:com.google.android.ext.services\n
            package:com.example.android.livecubes\npackage:com.android.providers.telephony\n
            package:com.google.android.googlequicksearchbox\npackage:com.android.provider.calendar\n
            package:com.android.providers.media\npackage:com.google.android.onetimeinitializer\n
            package:com.google.android.ext.shared\npackage:com.android.protips\n
            package:com.estrongs.android.pop\npackage:org.cryptomator.test\n
            package:com.dropbox.android\npackage:com.android.sdksetup\n
            package:com.ustwo.lwp\npackage:com.breel.geswallpapers";

        let mocked_apps = vec!["org.cryptomator".to_string(),
                               "com.example.android.livecubes".to_string(),
                               "com.estrongs.android.pop".to_string(),
                               "org.cryptomator.test".to_string(),
                               "com.dropbox.android".to_string(),
                               "com.ustwo.lwp".to_string(),
                               "com.breel.geswallpapers".to_string()];

        assert_that!(Backup::parse_list_apps(mocked_output.to_string()),
                                                                is(equal_to(mocked_apps)))
    }

    /*#[test]
    fn test_all_options_backup() {
        let options = BackupOptions::default("test")
            .with_applications()
            .with_shared_storage()
            .with_system_apps();

        let backup_result = Backup::backup(options);

        assert_that(backup_result.is_ok(), is(Ok(())))

        //let mocked_output = "List of devices attached\n\n\n".to_string();
        //assert_that!(Device::parse_devices(mocked_output), is(equal_to(Vec::new())));
    }*/
}
