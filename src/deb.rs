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
    pub architecture: String,
    pub file_name: String,
    pub deb_folder: String,
}

// Builder for DebianBinaryPkg
#[derive(Debug, PartialEq, Default)]
struct DraftBinaryPkg {
    name: Option<String>,
    source: Option<DebianSource>,
    version: Option<String>,
    architecture: Option<String>,
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
            architecture: self
                .architecture
                .context("Package is missing `Architecture` field")?,
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

pub fn parse_compressed_packages_db(packages_db: &[u8]) -> Result<Vec<DebianBinaryPkg>> {
    let reader = XzDecoder::new(packages_db);
    parse_packages_db(reader)
}

pub fn parse_packages_db<R: Read>(reader: R) -> Result<Vec<DebianBinaryPkg>> {
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
                    "Architecture" => {
                        trace!("Found field (architecture): {value:?}");
                        draft.architecture = Some(value.to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_package_db_sniffglue() {
        let data = br#"Package: sniffglue
Source: rust-sniffglue
Version: 0.14.0-2
Installed-Size: 2344
Maintainer: Debian Rust Maintainers <pkg-rust-maintainers@alioth-lists.debian.net>
Architecture: amd64
Depends: libc6 (>= 2.32), libgcc-s1 (>= 4.2), libpcap0.8 (>= 1.5.1), libseccomp2 (>= 0.0.0~20120605)
Description: Secure multithreaded packet sniffer
Multi-Arch: allowed
Built-Using: rust-nix (= 0.23.0-1), rust-pktparse (= 0.5.0-1), rust-seccomp-sys (= 0.1.3-1), rustc (= 1.56.0+dfsg1-2)
Description-md5: e7f1183e49341488d3bd8fbe63b63f37
X-Cargo-Built-Using: rust-aho-corasick (= 0.7.10-1), rust-ansi-term (= 0.12.1-1), rust-anyhow (= 1.0.44-2), rust-arrayvec (= 0.5.1-1), rust-atty (= 0.2.14-2), rust-base64 (= 0.13.0-1), rust-bitflags (= 1.2.1-1), rust-block-buffer (= 0.9.0-4), rust-block-padding (= 0.2.1-1), rust-bstr (= 0.2.17-1), rust-byteorder (= 1.4.3-2), rust-cfg-if-0.1 (= 0.1.10-2), rust-cfg-if (= 1.0.0-1), rust-clap (= 2.33.3-1), rust-cpuid-bool (= 0.1.2-4), rust-dhcp4r (= 0.2.0-1), rust-digest (= 0.9.0-1), rust-dirs-next (= 2.0.0-1), rust-dirs-sys-next (= 0.1.1-1), rust-dns-parser (= 0.8.0-1), rust-enum-primitive (= 0.1.1-1), rust-env-logger (= 0.9.0-1), rust-generic-array (= 0.14.4-1), rust-humantime (= 2.1.0-1), rust-itoa (= 0.4.3-1), rust-lazy-static (= 1.4.0-1), rust-lexical-core (= 0.4.8-3), rust-libc (= 0.2.103-1), rust-log (= 0.4.11-2), rust-memchr (= 2.4.1-1), rust-memoffset (= 0.6.4-1), rust-nix (= 0.23.0-1), rust-nom (= 5.0.1-4), rust-num-cpus (= 1.13.0-1), rust-num-traits (= 0.2.14-1), rust-opaque-debug (= 0.3.0-1), rust-pcap-sys (= 0.1.3-2), rust-phf (= 0.8.0-2), rust-phf-shared (= 0.8.0-1), rust-pktparse (= 0.5.0-1), rust-quick-error (= 1.2.3-1), rust-reduce (= 0.1.1-1), rust-regex-automata (= 0.1.8-2), rust-regex (= 1.5.4-1), rust-regex-syntax (= 0.6.25-1), rust-rusticata-macros (= 2.0.4-1), rust-ryu (= 1.0.2-1), rust-seccomp-sys (= 0.1.3-1), rust-serde (= 1.0.130-2), rust-serde-json (= 1.0.41-1), rust-sha2 (= 0.9.2-2), rust-siphasher (= 0.3.1-1), rust-static-assertions (= 1.1.0-1), rust-strsim (= 0.9.3-1), rust-structopt (= 0.3.20-1), rust-strum (= 0.19.2-1), rust-syscallz (= 0.15.0-1), rust-termcolor (= 1.1.0-1), rust-textwrap (= 0.11.0-1), rust-time (= 0.1.42-1), rust-tls-parser (= 0.9.2-3), rust-toml (= 0.5.8-1), rust-typenum (= 1.12.0-1), rust-unicode-width (= 0.1.8-1), rust-users (= 0.11.0-1), rust-vec-map (= 0.8.1-2), rustc (= 1.56.0+dfsg1-2)
Section: net
Priority: optional
Filename: pool/main/r/rust-sniffglue/sniffglue_0.14.0-2_amd64.deb
Size: 732980
MD5sum: 177f9229266ad5eef3fb42fff0c07345
SHA256: 448c781a9e594227bc9f0d6c65b8beba2b3add68d3583020de188d4cfa365b40

"#;
        let pkgs = parse_packages_db(&mut &data[..]).unwrap();
        assert_eq!(
            pkgs,
            &[DebianBinaryPkg {
                name: "sniffglue".to_string(),
                source: DebianSource {
                    name: "rust-sniffglue".to_string(),
                    version: None,
                },
                version: "0.14.0-2".to_string(),
                architecture: "amd64".to_string(),
                file_name: "sniffglue_0.14.0-2_amd64.deb".to_string(),
                deb_folder: "r/rust-sniffglue".to_string(),
            }]
        );
    }
}
