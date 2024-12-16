pub mod model;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::connection::SimpleConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Arc;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Clone)]
pub struct Database {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl Database {
    /// Creates a new `Database` instance, initializes the database, and seeds it.
    pub fn new(database_url: &str, seed_file: Option<&str>) -> Self {
        // Step 1: Create the database if it doesn't exist
        println!("Ensuring the database exists...");
        Self::create_database(database_url);

        // Step 2: Set up connection pool
        println!("Creating connection pool...");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        // Step 3: Run migrations
        println!("Running migrations...");
        let mut connection = pool
            .get()
            .expect("Failed to get a connection from the pool for migrations.");
        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations.");

        // Step 4: Seed the database (if a seed file is provided)
        if let Some(seed_file) = seed_file {
            println!("Seeding the database with: {}", seed_file);
            Self::seed_database(&mut connection, seed_file);
        }

        Self {
            pool: Arc::new(pool),
        }
    }

    /// Gets a connection from the pool.
    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool
            .get()
            .expect("Failed to get a connection from the pool.")
    }

    /// Creates the database if it doesn't exist.
    fn create_database(database_url: &str) {
        let (base_url, db_name) = Self::parse_database_url(database_url);

        let connection = PgConnection::establish(&base_url)
            .expect("Failed to connect to the PostgreSQL server.");

        let create_db_query = format!("CREATE DATABASE \"{}\";", db_name);
        if connection.batch_execute(&create_db_query).is_err() {
            println!(
                "Database '{}' already exists or creation failed. Skipping creation.",
                db_name
            );
        }
    }

    /// Seeds the database with a provided SQL file.
    fn seed_database(connection: &mut PgConnection, seed_file: &str) {
        use std::fs;

        let seed_sql = fs::read_to_string(seed_file)
            .expect("Failed to read the seed file.");

        connection
            .batch_execute(&seed_sql)
            .expect("Failed to seed the database.");
    }

    /// Parses the database URL to extract the base URL and database name.
    fn parse_database_url(database_url: &str) -> (String, String) {
        let parts: Vec<&str> = database_url.rsplitn(2, '/').collect();
        if parts.len() != 2 {
            panic!("Invalid database URL format.");
        }
        let base_url = parts[1].to_string();
        let db_name = parts[0].to_string();
        (base_url, db_name)
    }
}


