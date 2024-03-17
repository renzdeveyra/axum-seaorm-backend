use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
  let _db = Database::connect(DATABASE_URL).await?;

  let _db = &match _db.get_database_backend() {
    DbBackend::MySql => {
      _db.execute(Statement::from_string(
        _db.get_database_backend(),
        format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
      ))
      .await?;

      let url = format!("{}/{}", DATABASE_URL, DB_NAME);
      Database::connect(&url).await?
    }
    DbBackend::Postgres => {
      _db.execute(Statement::from_string(
        _db.get_database_backend(),
        format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
      ))
      .await?;
      _db.execute(Statement::from_string(
        _db.get_database_backend(),
        format!("CREATE DATABASE \"{}\";", DB_NAME),
      ))
      .await?;

      let url = format!("{}/{}", DATABASE_URL, DB_NAME);
      Database::connect(&url).await?
    }
    DbBackend::Sqlite => _db,
  };

  Ok(())
}

pub fn main() {
  if let Err(err) = block_on(run()) {
    panic!("{}", err);
  }
}