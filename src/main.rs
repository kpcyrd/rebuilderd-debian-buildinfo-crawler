#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod args;
mod buildinfo;
mod db;
mod deb;
mod errors;
mod html;
mod migrations;
mod models;
mod schema;
mod utils;

use crate::db::Database;
use crate::errors::*;
use clap::Parser;
use env_logger::Env;

#[derive(Debug, PartialEq)]
struct DebianSource {
    name: String,
    version: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();

    let logging = match args.verbose {
        0 => "info",
        1 => "info,rebuilderd_debian_buildinfo_crawler=debug",
        2 => "debug",
        _ => "debug,rebuilderd_debian_buildinfo_crawler=trace",
    };
    env_logger::init_from_env(Env::default().default_filter_or(logging));

    let db = Database::open(&args.database)?;
    let client = reqwest::Client::new();

    let packages_db = utils::read_path_or_url(&client, &args.packages_db).await?;
    let pkgs = deb::parse_packages_db(&packages_db).await?;
    info!("Found {} binary packages in index", pkgs.len());

    for pkg in pkgs {
        if let Some(buildinfo) = db.buildinfo_for_artifact_filename(&pkg.file_name)? {
            info!("Found buildinfo for {:?}: {:?}", pkg.file_name, buildinfo);
        } else {
            info!("Missing buildinfo for pkg: {:?}", pkg);
            let url = format!(
                "https://buildinfos.debian.net/buildinfo-pool/{}/",
                pkg.deb_folder
            );
            info!("Syncing buildinfos for source pkg: {:?}", url);
            match html::fetch_buildinfo_hrefs(&client, &url).await {
                Ok(buildinfos) => {
                    for buildinfo_href in buildinfos {
                        let url = format!("{}{}", url, buildinfo_href);

                        if db.buildinfo_url_cache(&url)?.is_none() {
                            info!("Download buildinfo file: {:?}", url);
                            let buildinfo = utils::fetch_http(&client, &url).await?;
                            let buildinfo = String::from_utf8(buildinfo)?;

                            debug!("Adding to cache for {:?}", url);
                            db.add_buildinfo(url, buildinfo)?;
                        }
                    }
                }
                Err(err) => warn!("Failed to fetch buildinfo directory listing: {:#}", err),
            }
            info!("Finished syncing buildinfos");
        }
    }

    Ok(())
}
