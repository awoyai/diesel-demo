use super::schema::posts;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i64,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub name: &'a str,
}

table! {
    sequences(id) {
        id -> BigInt,
    }
}

#[derive(QueryableByName)]
#[table_name = "sequences"]
pub struct Sequences {
    pub id: i64,
}
