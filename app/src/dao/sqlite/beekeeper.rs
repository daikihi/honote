use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = "beekeepers")]
pub struct Beekeeper {
    id: i64,
    beekeeper_name: String,
}

#[derive(Insertable)]
#[table_name = "beekeepers"]
pub struct NewBeekeeper<'a> {
    pub beekeeper_name: &'a str,
}

#[macro_use]
extern crate diesel;

pub fn create_beekeeper(conn: &SqliteConnection, beekeeper_name: &str) -> usize {
    let new_beekeeper = NewBeekeeper { beekeeper_name };
    use crate::schemas::sqlite::beekeeper::beekeepers;
    diesel::insert_into(beekeepers::table)
        .values(new_beekeeper)
        .execute(conn)
        .expect("Error saving new post");
}
