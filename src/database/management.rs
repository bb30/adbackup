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
    pub fn open_connection(device_id: &str) -> Result<DatabaseManager, Error> {
        let database_name = format!("{}.db", device_id);
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

    pub fn get_backup(&self, output_file: &str) -> Result<(), Error> {
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


// test: open dummy-db, insert dummy data and then other dummy data with same hash and check if getting the data results is equal to the second insert
