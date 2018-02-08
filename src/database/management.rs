use database::migration::DatabaseMigrator;
use database::rusqlite::Connection;
use devices::Device;
use failure::Error;

pub struct DatabaseManager
{
    version: u32,
    connection: Connection,
}

impl DatabaseManager
{
    fn get_database_version(conn: &Connection) -> Result<u32, Error>
    {
        conn.query_row(
            "SELECT version FROM adbackup_system",
            &[],
            |row| {
                row.get_checked(0)
            })?.map_err(|e| {
                Error::from(e)
            })
    }

    pub fn open_connection(device: &Device) -> Result<(), Error>
    {
        let conn = Connection::open(format!("{}.db", device.id))?;
        let version = match Self::get_database_version(&conn) {
            Ok(v) => v,
            Err(_) => 0 // FIXME find a way to only do this if e contains a SqliteFailure with string "table not found"
        };
    
        DatabaseMigrator::migrate(&conn, version)?;

        // FIXME persist this connection, i.e. return a instance of Handler on success
        Ok(())
    }
}
