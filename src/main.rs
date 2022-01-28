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
use rebuilderd_common::{PkgArtifact, PkgGroup};
use std::collections::HashMap;
use std::io::prelude::*;

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

    let mut without_buildinfo = Vec::new();
    let mut groups = HashMap::new();

    for pkg in pkgs {
        let buildinfo =
            if let Some(buildinfo) = db.buildinfo_for_artifact_filename(&pkg.file_name)? {
                info!("Found buildinfo for {:?} in database", pkg.file_name);
                debug!("Found buildinfo for {:?}: {:?}", pkg.file_name, buildinfo);
                buildinfo
            } else {
                info!("Missing buildinfo for pkg: {:?}", pkg);
                let url = format!(
                    "https://buildinfos.debian.net/buildinfo-pool/{}/",
                    pkg.deb_folder
                );

                let mut found = None;

                if !args.skip_crawl {
                    info!("Syncing buildinfos for source pkg: {:?}", url);
                    match html::fetch_buildinfo_hrefs(&client, &url).await {
                        Ok(mut buildinfos) => {
                            buildinfos.reverse();

                            for buildinfo_href in buildinfos {
                                let url = format!("{}{}", url, buildinfo_href);

                                if db.buildinfo_url_cache(&url)?.is_none() {
                                    // Download buildinfo file
                                    let buildinfo = utils::fetch_http(&client, &url).await?;
                                    let buildinfo = String::from_utf8(buildinfo)?;

                                    info!("Adding to cache for {:?}", url);
                                    let artifacts = db.add_buildinfo(url, buildinfo)?;
                                    if artifacts.contains(&pkg.file_name) {
                                        debug!("Buildinfo contained artifact we're looking for");
                                        let buildinfo = db
                                        .buildinfo_for_artifact_filename(&pkg.file_name)?
                                        .context(
                                            "Database doesn't contain artifact we just inserted",
                                        )?;
                                        found = Some(buildinfo);
                                        break;
                                    }
                                }
                            }
                        }
                        Err(err) => warn!("Failed to fetch buildinfo directory listing: {:#}", err),
                    }
                }

                if let Some(buildinfo) = found {
                    info!("Finished syncing buildinfos");
                    buildinfo
                } else {
                    warn!("Downloaded all buildinfos but none of them referenced our package");
                    without_buildinfo.push(pkg);
                    continue;
                }
            };

        let group = groups
            .entry(buildinfo.url.clone())
            .or_insert_with(|| (buildinfo, Vec::new()));
        group.1.push(pkg);
    }

    if !without_buildinfo.is_empty() {
        warn!(
            "Packages missing buildinfo files: {}",
            without_buildinfo.len()
        );
    }

    info!("Number of groups: {:?}", groups.len());
    info!(
        "Number of pkgs: {:?}",
        groups.iter().fold(0, |acc, (_, x)| acc + x.1.len())
    );

    info!("Generating build groups...");
    let mut out = Vec::new();
    for (_, (model, pkgs)) in groups {
        let buildinfo = model.content.parse::<buildinfo::Buildinfo>()?;

        let mut artifacts = Vec::new();
        for pkg in pkgs {
            let url = format!(
                "https://deb.debian.org/debian/pool/main/{}/{}",
                pkg.deb_folder, pkg.file_name
            );
            artifacts.push(PkgArtifact {
                name: pkg.name,
                version: pkg.version,
                url,
            });
        }

        out.push(PkgGroup {
            name: buildinfo.source,
            version: buildinfo.version,

            distro: args.distro.clone(),
            suite: args.suite.clone(),
            architecture: buildinfo.architecture,

            input_url: Some(model.url),
            artifacts,
        });
    }

    info!("Adding packages without buildinfo file...");
    let mut missing_groups = HashMap::new();
    for pkg in without_buildinfo {
        let src = pkg.source.name.clone();
        let version = pkg.source.version.as_deref().unwrap_or("0");

        let key = format!("{src:?}-{version:?}");

        let group = missing_groups
            .entry(key)
            .or_insert_with(|| (src, version.to_string(), Vec::new()));
        group.2.push(pkg);
    }

    for (_, (src, version, pkgs)) in missing_groups {
        let mut artifacts = Vec::new();
        for pkg in pkgs {
            let url = format!(
                "https://deb.debian.org/debian/pool/main/{}/{}",
                pkg.deb_folder, pkg.file_name
            );
            artifacts.push(PkgArtifact {
                name: pkg.name,
                version: pkg.version,
                url,
            });
        }

        let missing_buildinfo_url = format!(
            "https://buildinfos.debian.net/missing-buildinfo/{}/{}",
            src, version
        );
        out.push(PkgGroup {
            name: src,
            version,

            distro: args.distro.clone(),
            suite: args.suite.clone(),
            architecture: "???".to_string(),

            input_url: Some(missing_buildinfo_url),
            artifacts,
        });
    }

    info!("Sorting list...");
    out.sort_by(|a, b| a.name.cmp(&b.name).then(a.version.cmp(&b.version)));

    info!("Writing final json...");
    let mut stdout = std::io::stdout();
    serde_json::to_writer_pretty(&mut stdout, &out).ok();
    writeln!(stdout).ok();

    Ok(())
}
