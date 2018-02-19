use database::migration::DatabaseMigrator;
use database::rusqlite::Connection;
use failure::{Error, err_msg};
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

pub struct DatabaseManager {
    _version: u32,
    connection: Connection,
    name: String,
}

impl DatabaseManager {
    pub fn open_connection(name: &str) -> Result<DatabaseManager, Error> {
        let database_name = match name.ends_with(".db") {
            true => String::from(name),
            false => format!("{}.db", name)
        };
        
        let conn = Connection::open(&database_name)?;
        let version = match DatabaseMigrator::get_database_version(&conn) {
            Ok(v) => v,
            Err(_) => 0 // FIXME v1 find a way to only do this if e contains a SqliteFailure with string "table not found"
        };
    
        DatabaseMigrator::migrate(&conn, version)?;

        Ok(DatabaseManager {connection: conn, _version: version, name: database_name} )
    }

    pub fn insert_data(&self, input_file: &str) -> Result<(), Error> {
        if !Path::new(&self.name).exists() {
            return Err(err_msg("Could not open database"));
        }

        // check how many files there are in the sqlite database (versioning)
        let count: u32 = self.connection.query_row(
            "SELECT COUNT(version) FROM device_data",
            &[],
            |row| {
                row.get_checked(0)
            })?.map_err(|e| {
                Error::from(e)
            })?;

        // insert file as blob into table (we can use a constant for the data_hash as incremental backups (for which we need to identify single files by their hash) are not yet implementable)
        let mut backup_file = File::open(input_file)?;
        let mut file_bytes = Vec::new();
        backup_file.read_to_end(&mut file_bytes)?;

        self.connection.execute("INSERT INTO device_data (data_hash, version, data)
            VALUES (?1, ?2, ?3)",
            &[&"const_hash", &(count + 1), &file_bytes])?;

        Ok(())
    }

    pub fn get_latest_backup(&self, output_file: &str) -> Result<(), Error> {
        if !Path::new(&self.name).exists() {
            return Err(err_msg("Could not open database"));
        }

        // get blob from database and save as file
        let data: Vec<u8> = self.connection.query_row(
            "SELECT data FROM device_data ORDER BY version DESC LIMIT 1",
            &[],
            |row| {
                row.get_checked(0)
            })?.map_err(|e| {
                Error::from(e)
            })?;

        let mut file = File::create(output_file)?;
        file.write_all(&data)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use database::management::DatabaseManager;
    use database::migration::CURRENT_VERSION;
    use std::fs::{copy, File, remove_file};
    use std::io::{Read, Write};

    #[test]
    fn test_data_insertion_and_retrieval() {
        let current_db_name = format!("tests/test_databases/dummy_db_v{}.db", CURRENT_VERSION);
        let temp_db = "847f7b63b0325d18e3b628e4604b6be4.db"; // md5 of 'test_data_insertion_and_retrieval'

        assert!(copy(current_db_name, temp_db).is_ok());

        let first_data_file = "356779463358b0ea3c8d45c975c54981";
        let second_data_file = "9905268f4a804227b64bd01716b599f6";
        let output_data_file = "9c462f205ae62c436ca700332dd009b4";

        {
            let db_manager = DatabaseManager::open_connection(temp_db).unwrap();

            let mut first_data = File::create(&first_data_file).unwrap();
            assert!(first_data.write(&vec![00, 01, 02]).is_ok());

            assert!(db_manager.insert_data(&first_data_file).is_ok());

            
            let mut second_data = File::create(&second_data_file).unwrap();
            assert!(second_data.write(&vec![03, 04, 05]).is_ok());

            assert!(db_manager.insert_data(&second_data_file).is_ok());
            
            assert!(db_manager.get_latest_backup(&output_data_file).is_ok());
        }

        let mut file_wanted = File::open(&second_data_file).unwrap();
        let mut data_wanted = Vec::new();
        assert!(file_wanted.read_to_end(&mut data_wanted).is_ok());

        let mut file_result = File::open(&output_data_file).unwrap();
        let mut data_result = Vec::new();
        assert!(file_result.read_to_end(&mut data_result).is_ok());

        assert_eq!(data_wanted, data_result);

        assert!(remove_file(&temp_db).is_ok());
        assert!(remove_file(&first_data_file).is_ok());
        assert!(remove_file(&second_data_file).is_ok());
        assert!(remove_file(&output_data_file).is_ok());
    }
}
