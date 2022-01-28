use crate::errors::*;
use std::io::prelude::*;
use std::io::BufReader;
use xz2::read::XzDecoder;

#[derive(Debug, PartialEq)]
pub struct DebianSource {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct DebianBinaryPkg {
    pub name: String,
    pub source: DebianSource,
    pub version: String,
    pub file_name: String,
    pub deb_folder: String,
}

// Builder for DebianBinaryPkg
#[derive(Debug, PartialEq, Default)]
struct DraftBinaryPkg {
    name: Option<String>,
    source: Option<DebianSource>,
    version: Option<String>,
    file_name: Option<String>,
    deb_folder: Option<String>,
}

impl TryInto<DebianBinaryPkg> for DraftBinaryPkg {
    type Error = Error;

    fn try_into(self) -> Result<DebianBinaryPkg> {
        let name = self.name.context("Package is missing `Package` field")?;

        let source = self.source.unwrap_or_else(|| DebianSource {
            name: name.to_string(),
            version: None,
        });

        let bin = DebianBinaryPkg {
            name,
            source,
            version: self.version.context("Package is missing `Version` field")?,
            file_name: self
                .file_name
                .context("Package is misisng `Filename` field")?,
            deb_folder: self
                .deb_folder
                .context("Package is missing `Filename` field")?,
        };
        Ok(bin)
    }
}

pub async fn parse_packages_db(packages_db: &[u8]) -> Result<Vec<DebianBinaryPkg>> {
    let reader = XzDecoder::new(packages_db);
    let reader = BufReader::new(reader);

    let mut out = Vec::new();
    let mut draft = DraftBinaryPkg::default();

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            let bin = draft.try_into()?;
            trace!("Learned debian binary pkg: {:?}", bin);
            out.push(bin);
            draft = DraftBinaryPkg::default();
        } else if !line.starts_with(' ') {
            if let Some((key, value)) = line.split_once(": ") {
                trace!("Parsed line in package database: {key:?} => {value:?}");
                match key {
                    "Package" => {
                        trace!("Found field (package): {value:?}");
                        draft.name = Some(value.to_string());
                    }
                    "Source" => {
                        trace!("Found field (source): {value:?}");
                        let (name, version) = value
                            .rsplit_once(' ')
                            .map(|(name, version)| {
                                let version =
                                    version.trim_matches(|c| c == '(' || c == ')').to_string();
                                (name.to_string(), Some(version))
                            })
                            .unwrap_or_else(|| (value.to_string(), None));

                        let s = DebianSource {
                            name: name.to_string(),
                            version,
                        };
                        trace!("Source reference: {s:?}");
                        draft.source = Some(s);
                    }
                    "Version" => {
                        trace!("Found field (version): {value:?}");
                        draft.version = Some(value.to_string());
                    }
                    "Filename" => {
                        let (folder, file_name) = value.rsplit_once('/').unwrap_or(("", value));
                        trace!(
                            "Found value of filename field ({value:?}), resolved to filename {file_name:?}"
                        );
                        draft.file_name = Some(file_name.to_string());
                        let (_, folder) = folder.split_once('/').unwrap_or(("", value));
                        let (_, folder) = folder.split_once('/').unwrap_or(("", value));
                        draft.deb_folder = Some(folder.to_string());
                    }
                    _ => (),
                }
            }
        }
    }

    Ok(out)
}
