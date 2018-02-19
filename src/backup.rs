use failure::Error;

use adb_command::AdbCommand;

#[derive(Debug, PartialEq, Clone)]
pub struct BackupOptions<'a> {
    device_id: &'a str,
    applications: &'a str,
    shared_storage: &'a str,
    system_apps: &'a str,
    only_specified_app: &'a str,
}

impl<'a> BackupOptions<'a> {
    pub fn default(device_id: &'a str) -> Self {
        BackupOptions {
            device_id: device_id,
            applications: "-noapk",
            shared_storage: "-noshared",
            system_apps: "-nosystem",
            only_specified_app: "-all",
        }
    }

    pub fn with_applications(self) -> Self {
        BackupOptions {
            applications: "-apk",
            ..self
        }
    }

    pub fn with_shared_storage(self) -> Self {
        BackupOptions {
            shared_storage: "-shared",
            ..self
        }
    }

    pub fn with_system_apps(self) -> Self {
        BackupOptions {
            system_apps: "-system",
            ..self
        }
    }

    pub fn with_only_specified_app(self, apps: &'a str) -> Self {
        BackupOptions {
            only_specified_app: apps,
            ..self
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Backup {}

impl Backup {
    pub fn backup(backup_options: BackupOptions) -> Result<(), Error> {
        let command_args: Vec<&str> = vec![
            &backup_options.applications,
            &backup_options.shared_storage,
            &backup_options.system_apps,
            &backup_options.applications,
            &backup_options.only_specified_app,
        ];

        let backup_name = format!("{}.ab", backup_options.device_id);
        let adb_command = AdbCommand::command("backup")
                .with_args(command_args)
                .with_arg("-f").with_arg(&backup_name)
                .with_device_id(Some(backup_options.device_id));

        AdbCommand::execute(adb_command)?;
        
        Ok(())
    }

    pub fn list_apps(device_id: Option<&str>) -> Result<Vec<String>, Error> {
        let args: Vec<&str> = vec!["pm", "list", "packages"];

        let output = AdbCommand::command("shell")
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
