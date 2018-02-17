use failure::{err_msg, Error};
use std::fs::{File, read_dir, remove_dir_all, remove_file};
use std::path::Path;
use std::process::{Command, Stdio};
use tar::{Archive, Builder};
use std::env;

// WARNING: this currently uses a external tool (abe.jar) to extract the android backup
// to files, this is because there is (at the moment!) no suitable rust crypto crate which we can use for this, see: 
// https://github.com/briansmith/ring/issues/573 and https://github.com/briansmith/ring/issues/588
// DANGER: also, the tar crate is not able to extract files from archives which have invalid file names on the current OS,
// as a result of this a few files from every backup get lost irrevocably -> the file extraction should not be used before this is fixed
// (=> create a PR in the tar crate, possible solution: vector / map with chars that need a replacement char on current OS)

pub struct Extract;

impl Extract {
    pub fn extract_to_folder(input: &str, output: &str, password: &str) -> Result<(), Error>
    {
        let output_archive_name = format!("{}.tar", output);
        Self::execute_abe("unpack", input, &output_archive_name, password)?;

        let archive_file = File::open(&output_archive_name)?;
        let mut archive = Archive::new(archive_file);

        for folder in archive.entries()?
        {
            // extract archive entries to output dir
            let mut unwrapped_folder = folder?;
            if let Err(e) = unwrapped_folder.unpack_in(output)
            {
                trace!("{}", e);
            }
        }

        // delete intermediary archive
        remove_file(&output_archive_name)?;

        // make a tar archive out of every subfolder of the backup
        for root_folder in read_dir(output)?
        {
            for sub_folder in read_dir(root_folder?.path())?
            {
                let unwrapped_sub_folder = sub_folder?;
                if unwrapped_sub_folder.path().is_dir()
                {
                    let sub_archive_name = format!("{}.tar", unwrapped_sub_folder.path().display());
                    let sub_archive_file = File::create(&sub_archive_name)?;
                    let mut sub_archive = Builder::new(sub_archive_file);
                    if let Some(dir) = unwrapped_sub_folder.path().file_name()
                    {
                        if let Some(dir_str) = dir.to_str()
                        {
                            sub_archive.append_dir_all(dir_str, unwrapped_sub_folder.path())?;

                            // the plain files are not needed anymore after they have been appended to the archive
                            remove_dir_all(unwrapped_sub_folder.path())?;
                        }
                    }
                    sub_archive.finish()?;
                }
            }
        }

        // FIXME it might be necessary to generate a list of the original tar entries to later (re)generate the tar archive from the list (as suggested by the android backup extractor (abe))?

        Ok(())
    }

    pub fn pack_from_folder(input: &str, output: &str, password: &str) -> Result<(), Error>
    {
        let input_archive = format!("{}.tar", input);
        let archive_file = File::create(&input_archive)?;
        let mut archive = Builder::new(archive_file);

        // iterate over files from input folder (also tar archives), extract them and copy them to new archive
        // extract package tar archives to folders
        for root_folder in read_dir(input)?
        {
            let unwrapped_root_folder = root_folder?;
            for sub_folder in read_dir(unwrapped_root_folder.path())?
            {
                let unwrapped_sub_folder = sub_folder?;
                if unwrapped_sub_folder.path().is_file()
                {
                    let sub_archive_file = File::open(&unwrapped_sub_folder.path())?;
                    let mut archive = Archive::new(sub_archive_file);
                    for folder in archive.entries()?
                    {
                        // extract archive entries to dir
                        let mut unwrapped_folder = folder?;
                        if let Err(e) = unwrapped_folder.unpack_in(unwrapped_root_folder.path())
                        {
                            trace!("{}", e);
                        }
                    }

                    // remove tar archive so it doesn't get copied to the final archive
                    remove_file(unwrapped_sub_folder.path())?;
                }
            }

            if let Some(dir) = unwrapped_root_folder.path().file_name()
            {
                if let Some(dir_str) = dir.to_str()
                {
                    archive.append_dir_all(dir_str, unwrapped_root_folder.path())?;
                }
            }
        }
        archive.finish()?;

        // plain files are not necessary anymore
        remove_dir_all(input)?;

        Self::execute_abe("pack", &input_archive, output, password)?;
        Ok(())
    }

    fn execute_abe(mode: &str, input: &str, output: &str, password: &str) ->Result<String, Error>
    {
        let abe_path = env::current_dir()?.join("abe.jar");
        if !Path::new(&abe_path).exists()
        {
            return Err(err_msg("Cannot pack/unpack backup without abe.jar"));
        }

        let mut command = Command::new("java");
        command.arg("-jar").arg(abe_path).arg(mode).arg(input).arg(output).arg(password);

        trace!("Extracting backup");

        let output = command.stderr(Stdio::piped()).output()?;
        if output.status.success()
        {
            let output_message = String::from_utf8_lossy(&output.stdout);
            trace!("output message from {:?}: {}", command, output_message);
            Ok(output_message.to_string())
        }
        else
        {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(err_msg(format!(
                "Error executing {:?}.\n {}",
                command, error_message
            )));
        }
    }
}
