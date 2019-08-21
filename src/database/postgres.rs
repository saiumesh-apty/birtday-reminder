use lazy_static;
use r2d2::PooledConnection;
use r2d2_postgres::{ PostgresConnectionManager, TlsMode };
use std::sync::{ Arc, Mutex };
use std::{env};

#[derive(Debug)]
struct Database {
    postgres: Arc<Mutex<PooledConnection<PostgresConnectionManager>>>
}

type PGInstance = Arc<Mutex<PooledConnection<PostgresConnectionManager>>>;

fn get_database_url() -> String {
    let dev_db =  String::from("postgres://postgres:postgres@localhost:5432/actix-web");
    match env::var("DATABASE_URL") {
        Ok(env) => env,
        Err(_err) => dev_db
    }
}

// Connects to the Postgres Database
fn connect_pg() -> PooledConnection<PostgresConnectionManager> {
    println!("db_url {:?}", get_database_url());
    let manager = PostgresConnectionManager::new(get_database_url(), TlsMode::None)
    .unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();
    let connection = pool.get().unwrap();
    return connection;
}

lazy_static! {
    static ref LOL: Database = Database {
        postgres: Arc::new(Mutex::new(connect_pg()))
    };
}

impl Database {
    fn get_instance(&self) -> &PGInstance  {
        &self.postgres
    }
}

pub fn get_instance() -> PGInstance {
    LOL.get_instance().clone()
}

pub fn connect_postgres() {
    println!("hello {:?}", *LOL);
}
