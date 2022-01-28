use crate::buildinfo;
use crate::errors::*;
use crate::migrations;
use crate::models::artifact::{Artifact, NewArtifact};
use crate::models::buildinfo::{Buildinfo, NewBuildinfo};
use crate::schema::*;
use diesel::connection::SimpleConnection;
use diesel::prelude::*;

pub struct Database {
    sqlite: diesel::SqliteConnection,
}

impl Database {
    pub fn open(path: &str) -> Result<Database> {
        let sqlite = SqliteConnection::establish(path).context("Failed to connect to database")?;

        sqlite.batch_execute(
            "
            PRAGMA busy_timeout = 10000;        -- sleep if the database is busy
            PRAGMA foreign_keys = ON;           -- enforce foreign keys
        ",
        )?;

        sqlite.batch_execute("
            PRAGMA journal_mode = WAL;          -- better write-concurrency
            PRAGMA synchronous = NORMAL;        -- fsync only in critical moments
            PRAGMA wal_autocheckpoint = 1000;   -- write WAL changes back every 1000 pages, for an in average 1MB WAL file. May affect readers if number is increased
            PRAGMA wal_checkpoint(TRUNCATE);    -- free some space by truncating possibly massive WAL files from the last run.
        ")?;

        debug!("Running missing migrations (if any)");
        migrations::run(&sqlite).context("Failed to run migrations")?;

        Ok(Database { sqlite })
    }

    pub fn buildinfo_for_artifact_filename(&self, my_filename: &str) -> Result<Option<Buildinfo>> {
        use crate::schema::artifacts::dsl::*;
        let artifact = artifacts
            .filter(file_name.eq(my_filename))
            .first::<Artifact>(&self.sqlite)
            .optional()?;

        if let Some(artifact) = artifact {
            use crate::schema::buildinfos::dsl::*;
            let buildinfo = buildinfos
                .filter(id.eq(artifact.buildinfo_id))
                .first(&self.sqlite)
                .optional()?;
            Ok(buildinfo)
        } else {
            Ok(None)
        }
    }

    pub fn buildinfo_url_cache(&self, my_url: &str) -> Result<Option<Buildinfo>> {
        use crate::schema::buildinfos::dsl::*;
        let buildinfo = buildinfos
            .filter(url.eq(my_url))
            .first(&self.sqlite)
            .optional()?;
        Ok(buildinfo)
    }

    pub fn add_buildinfo(&self, url: String, content: String) -> Result<Vec<String>> {
        let buildinfo = content.parse::<buildinfo::Buildinfo>()?;

        let mut out = Vec::new();
        self.sqlite.transaction::<_, Error, _>(|| {
            let my_url = url.clone();

            // insert buildinfo
            diesel::insert_into(buildinfos::table)
                .values(NewBuildinfo { url, content })
                .execute(&self.sqlite)?;

            // get buildinfo row id
            let buildinfo_id = {
                use crate::schema::buildinfos::dsl::*;
                let buildinfo_row = buildinfos
                    .filter(url.eq(&my_url))
                    .first::<Buildinfo>(&self.sqlite)?;
                buildinfo_row.id
            };

            // insert artifacts too
            for artifact in buildinfo.artifacts {
                out.push(artifact.to_string());
                diesel::insert_into(artifacts::table)
                    .values(NewArtifact {
                        file_name: artifact,
                        buildinfo_id,
                    })
                    .execute(&self.sqlite)?;
            }

            Ok(())
        })?;

        Ok(out)
    }
}
