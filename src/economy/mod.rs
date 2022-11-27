pub mod economy {
    use chrono::DateTime;
    use rusqlite::{Connection, Result};

    pub async fn convert_economy_database(old_db_path: &str, new_db_path: &str) -> Result<()> {
        let old_economy =  Connection::open(old_db_path)?;
        let new_economy =  Connection::open(new_db_path)?;
        let users = fetch_old_users(&old_economy).await?;
        create_new_table(&new_economy).await?;
        for mut user in users {
            if user.balance < 300 {
                user.balance = 300; // legacy bonus ^.^
            }
            add_new_user(&new_economy, user).await?;
        }
        old_economy.close().expect("Failed to close database");
        new_economy.close().expect("Failed to close database");
        println!("Economy database converted!");
        Ok(())
    }

    /// Struct to represent a user in our databases
    pub struct User {
        id: String,
        balance: u32,
        created: i64,
        updated: i64,
    }

    /// Creates a table named 'Economy' in the new format
    async fn create_new_table(db: &Connection) -> Result<usize> {
        db.execute(
            "CREATE TABLE IF NOT EXISTS Economy (
              id VARCHAR(32)    UNIQUE,
              balance           INTEGER,
              daily             DATETIME,
              work              DATETIME,
              created           DATETIME,
              updated           DATETIME
             )",
            (),
        )
    }

    /// Add a user to the new database
    async fn add_new_user(db: &Connection, user: User) -> Result<usize> {
        db.execute(
            "INSERT INTO Economy VALUES(
                ?1,
                ?2,
                0,
                0,
                ?3,
                ?4
            )",
            (
                user.id,
                user.balance,
                user.created,
                user.updated,
            )
        )
    }

    /// Fetch all users from database
    async fn fetch_old_users(db: &Connection) -> Result<Vec<User>>{
        let mut stmt = db.prepare("SELECT * FROM Economies")?;
        let mut rows = stmt.query([])?;
        let mut users:Vec<User> = Vec::new();
        while let Some(row) = rows.next()? {
            users.push(User {
                id: (&row).get(1)?,
                balance: (&row).get(2)?,
                created: get_timestamp_from_string(row.get(4)?),
                updated: get_timestamp_from_string(row.get(5)?),
            })
        };
        Ok(users)
    }

    // Convert unconventionally formatted rfc3339 format date from a String to u32
    fn get_timestamp_from_string(date: String) -> i64 {
        match DateTime::parse_from_rfc3339(date.replacen(" ", "T", 1).replace(" ", "").as_str()) {
            Ok(datetime) => datetime.timestamp_millis(),
            Err(_) => 0
        }
    }
}