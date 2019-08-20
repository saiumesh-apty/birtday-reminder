use r2d2::PooledConnection;
// use std::sync::MutexGuard;
use std::sync::{ Arc, Mutex };
use r2d2_postgres::PostgresConnectionManager;

pub type Postgres = Arc<Mutex<PooledConnection<PostgresConnectionManager>>>;
// pub type PostgresBorrowed<'a> = MutexGuard<'a, PooledConnection<PostgresConnectionManager>>;