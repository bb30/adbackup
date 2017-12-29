use failure::Error;

use adb_command::AdbCommand;

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
        let command_args = vec![
            backup_options.applications.clone(),
            backup_options.shared_storage.clone(),
            backup_options.system_apps.clone(),
            backup_options.applications.clone(),
            backup_options.only_specified_apps.clone(),
        ];

        let adb_command = AdbCommand::command("backup".to_string())
            .with_args(command_args)
            .with_device_id(backup_options.device_id);

        AdbCommand::execute(adb_command)?;

        Ok(())
    }

    pub fn list_apps(device_id: Option<String>) -> Result<Vec<String>, Error> {
        let args = vec!["pm".to_string(), "list".to_string(), "packages".to_string()];

        let output = AdbCommand::command("shell".to_string())
            .with_args(args)
            .with_device_id(device_id)
            .execute()?;

        return Ok(Backup::parse_list_apps(output));
    }

    fn parse_list_apps(command_response: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        command_response
            .split('\n')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|x| {
                !x.contains("package:com.android.") && !x.contains("package:com.google.android.")
            })
            .for_each(|package| {
                let split_package = package.split(':').collect::<Vec<&str>>();

                if let Some(package_name) = split_package.get(1) {
                    result.push(package_name.to_string());
                }
            });

        result
    }
}

#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use backup::Backup;

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

        let mocked_apps = vec![
            "org.cryptomator".to_string(),
            "com.example.android.livecubes".to_string(),
            "com.estrongs.android.pop".to_string(),
            "org.cryptomator.test".to_string(),
            "com.dropbox.android".to_string(),
            "com.ustwo.lwp".to_string(),
            "com.breel.geswallpapers".to_string(),
        ];

        assert_that!(
            Backup::parse_list_apps(mocked_output.to_string()),
            is(equal_to(mocked_apps))
        )
    }
}
