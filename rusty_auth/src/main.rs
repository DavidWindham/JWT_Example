#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};
use db::establish_connection;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

use dotenv::dotenv;

mod auth;
mod consts;
mod db;
mod response;
mod schema;
mod user;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().ok();

    // load all the ENV vars to ensure they're set
    env::var("ACCESS_TOKEN_EXPIRE_SECONDS")
        .expect("DATABASE_URL must be set")
        .parse::<i64>()
        .unwrap();
    env::var("REFRESH_TOKEN_EXPIRE_SECONDS")
        .expect("DATABASE_URL must be set")
        .parse::<i64>()
        .unwrap();
    env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");
    env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");

    // set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let _db_connection = &mut establish_connection();

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(pool.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(user::register_new_user)
            .service(user::login_user)
            .service(auth::verify_token)
            .service(auth::refresh_token)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
