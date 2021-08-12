use diesel::{
    connection::Connection,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn run_migrations(db_url: &str){
    embed_migrations!();
    let conn = PgConnection::establish(db_url).expect("Error with connecting to the database");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
    .expect("Error with running migrations");
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>>{
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error with building a connection pool")
}