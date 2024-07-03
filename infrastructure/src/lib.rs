use diesel::{pg::PgConnection, r2d2::{Pool, ConnectionManager}};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> DBPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}