use database::rusqlite::Connection;
use failure::Error;

static CURRENT_VERSION: u32 = 1;

#[derive(Debug, Fail)]
pub enum MigratorError {
    #[fail(display = "unknown database version: {}", version)]
    UnknownDatabaseVersion {
        version: u32,
    },

    #[fail(display = "no migration function for version {} implemented", version)]
    NoMigrationFunction {
        version: u32,
    }
}

pub struct DatabaseMigrator;

impl DatabaseMigrator {
    pub fn get_database_version(conn: &Connection) -> Result<u32, Error> {
        conn.query_row(
            "SELECT version FROM adbackup_system",
            &[],
            |row| {
                row.get_checked(0)
            })?.map_err(|e| {
                Error::from(e)
            })
    }

    pub fn migrate(conn: &Connection, version: u32) -> Result<(), Error> {
        if version > CURRENT_VERSION {
            // FIXME v1 is there a better way to convert between these types?
            return Err(Error::from(MigratorError::UnknownDatabaseVersion { version }))
        }

        let mut ver = version;
        while ver < CURRENT_VERSION {
            match ver {
                0 => Self::to_one_from_none(conn)?,
                _ => return Err(Error::from(MigratorError::NoMigrationFunction { version }))
            };

            ver += 1;
        }
        Ok(())
    }

    // no database -> v1
    fn to_one_from_none(conn: &Connection) -> Result<(), Error> {
        conn.execute("CREATE TABLE adbackup_system (version INTEGER NOT NULL)", &[])?;
        conn.execute("INSERT INTO adbackup_system VALUES(1)", &[])?;
        conn.execute("CREATE TABLE device_data (
            data_hash       TEXT NOT NULL,
            version         INTEGER NOT NULL,
            data            BLOB,
            date_created    INTEGER NOT NULL DEFAULT CURRENT_TIME,
            PRIMARY KEY(data_hash, version)
            )", &[])?;

        Ok(())
    }
}

// test: if migration from 0 to 1 works -> push dummy-db to repo and compare it to one created by the migrator (or just check if tables exist for v1), also take newest dummy-db and test if getting the db version results in the current version (constant)
