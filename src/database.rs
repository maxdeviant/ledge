use rocket_contrib::databases::diesel;

pub mod schema;

#[database("postgres")]
pub struct DatabaseConnection(diesel::PgConnection);
