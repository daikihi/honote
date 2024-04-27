use dao::sqlite::Beekeeper;
use diesel::prelude::*;

#[derive(Associations, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(belongs_to(Beekeeper))]
pub struct Honey {
    id: i64,
    honey_name: String,
    beekeeper_id: i64,
}
