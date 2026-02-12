use sqlx::migrate::MigrateDatabase;
use sqlx::{Pool, Sqlite, SqlitePool};

pub const DEFAULT_DB_URL: &str = "sqlite://data.db";
pub const IN_MEMORY_DB_URL: &str = "sqlite::memory:";

async fn init_db_file(url: &str) {
    if !Sqlite::database_exists(url).await.unwrap_or(false) {
        println!("Creating database {}", url);
        match Sqlite::create_database(url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

async fn connect_db(url: &str) -> Pool<Sqlite> {
    SqlitePool::connect(url).await.unwrap()
}
// should I use the file system or not? hmmm db tables are not likely to change or be added a lot
// there is an argument for keeping them as consts in the code so the binary doesn't need to load the file
async fn migrate_db(db: &Pool<Sqlite>) {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(db)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);
}

pub async fn init_db(url: &str) -> Pool<Sqlite> {
    init_db_file(url).await;
    let db = connect_db(url).await;
    migrate_db(&db).await;
    db
}
