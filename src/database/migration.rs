use database::rusqlite::Connection;
use failure::Error;

pub static CURRENT_VERSION: u32 = 1;

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

    pub fn migrate(conn: &Connection, current_version: u32) -> Result<(), Error> {
        if current_version > CURRENT_VERSION {
            return Err(Error::from(MigratorError::UnknownDatabaseVersion { version: current_version }))
        }

        let mut ver = current_version;
        while ver < CURRENT_VERSION {
            match ver {
                0 => Self::upgrade_to_one_from_none(conn)?,
                _ => return Err(Error::from(MigratorError::NoMigrationFunction { version: ver }))
            };

            ver += 1;
        }
        Ok(())
    }

    // no database -> v1
    fn upgrade_to_one_from_none(conn: &Connection) -> Result<(), Error> {
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

#[cfg(test)]
mod tests {
    use database::rusqlite::Connection;
    use database::migration::{DatabaseMigrator, CURRENT_VERSION};
    use std::fs::remove_file;

    #[test]
    fn test_version_retrieval() {
        let current_db_name = format!("tests/test_databases/dummy_db_v{}.db", CURRENT_VERSION);
        let conn = Connection::open(&current_db_name).unwrap();
        let ver = DatabaseMigrator::get_database_version(&conn);

        assert!(ver.is_ok());
        assert_eq!(ver.unwrap(), CURRENT_VERSION);
    }

    #[test]
    fn test_migration_one_from_zero() {
        let temp_db = "bf9d29b4cac2a22e1395e1244ac3ed74.db"; // md5 of 'test_migration_one_from_zero'
        let conn = Connection::open(&temp_db).unwrap();

        assert!(DatabaseMigrator::migrate(&conn, 0).is_ok());

        assert_eq!(DatabaseMigrator::get_database_version(&conn).unwrap(), 1);

        assert!(conn.close().is_ok());
        assert!(remove_file(&temp_db).is_ok());
    }

    #[test]
    fn test_migration_unknown_version() {
        let temp_db = "5b40b7e6711716091e89a691f1ab726b.db"; // md5 of 'test_migration_unknown_version'
        let conn = Connection::open(&temp_db).unwrap();

        assert_eq!(format!("{}", DatabaseMigrator::migrate(&conn, CURRENT_VERSION + 1).unwrap_err()), format!("unknown database version: {}", CURRENT_VERSION + 1));

        assert!(conn.close().is_ok());
        assert!(remove_file(&temp_db).is_ok());
    }
}
