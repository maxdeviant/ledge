#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod database;
mod graphql;
mod routes;

use dotenv::dotenv;

use self::database::DatabaseConnection;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(DatabaseConnection::fairing())
        .mount("/", routes::graphql::routes())
        .launch();
}
