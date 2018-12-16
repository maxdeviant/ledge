use rocket_contrib::databases::diesel;

#[database("postgres")]
pub struct DatabaseConnection(diesel::PgConnection);
