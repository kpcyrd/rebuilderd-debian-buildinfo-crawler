use crate::schema::*;

#[derive(Identifiable, Queryable, AsChangeset, Clone, PartialEq, Debug)]
#[table_name = "artifacts"]
pub struct Artifact {
    pub id: i32,
    pub file_name: String,
    pub buildinfo_id: i32,
}

#[derive(Insertable, Clone, PartialEq, Debug)]
#[table_name = "artifacts"]
pub struct NewArtifact {
    pub file_name: String,
    pub buildinfo_id: i32,
}
