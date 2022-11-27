mod economy;

use chrono::DateTime;
use rusqlite::{Connection, Result};
use crate::economy::economy::convert_economy_database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Running...");
    println!("Converting economy database...");
    convert_economy_database("./database.sqlite", "economy.db").await?;
    Ok(())
}
