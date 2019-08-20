use lazy_static;
use r2d2::PooledConnection;
use r2d2_postgres::{ PostgresConnectionManager, TlsMode };
use std::sync::{ Arc, Mutex };

#[derive(Debug)]
struct Database {
    postgres: Arc<Mutex<PooledConnection<PostgresConnectionManager>>>
}

type PGInstance = Arc<Mutex<PooledConnection<PostgresConnectionManager>>>;

// Connects to the Postgres Database
fn connect_pg() -> PooledConnection<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(
        "postgres://pqxijtttcyssbn:3c84e676998c167df6dfdccaccffd438338d4a4f02174f3ce60c8464815972e1@ec2-54-221-212-126.compute-1.amazonaws.com:5432/d3in0914d0sos5",
        TlsMode::None,
    )
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
