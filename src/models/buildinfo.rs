use crate::schema::*;

#[derive(Identifiable, Queryable, AsChangeset, Clone, PartialEq, Debug)]
#[table_name = "buildinfos"]
pub struct Buildinfo {
    pub id: i32,
    pub url: String,
    pub content: String,
}

#[derive(Insertable, Clone, PartialEq, Debug)]
#[table_name = "buildinfos"]
pub struct NewBuildinfo {
    pub url: String,
    pub content: String,
}
