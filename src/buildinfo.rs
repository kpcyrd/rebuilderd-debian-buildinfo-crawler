use crate::errors::*;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Buildinfo {
    pub source: String,
    pub version: String,
    pub artifacts: HashSet<String>,
}

impl FromStr for Buildinfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut source = None;
        let mut version = None;
        let mut artifacts = HashSet::new();

        let mut section = None;
        for line in s.split('\n') {
            // Set section
            if let Some(s) = line.strip_suffix(':') {
                section = Some(s.to_string());
            }

            if line.starts_with(' ') {
                // we're inside of a section
                if let Some(section) = &section {
                    if section.starts_with("Checksums-") {
                        if let Some(artifact) = artifact_from_checksum_line(line) {
                            artifacts.insert(artifact);
                        }
                    }
                }
            } else {
                // Regular key/value
                if let Some((key, value)) = line.split_once(": ") {
                    match key {
                        "Source" => {
                            let (src, _) = value.split_once(' ').unwrap_or((value, ""));
                            source = Some(src.to_string());
                        }
                        "Version" => version = Some(value.to_string()),
                        _ => (),
                    }
                }
            }
        }

        Ok(Buildinfo {
            source: source.context("Missing `source` field in buildinfo")?,
            version: version.context("Missing `version` field in buildinfo")?,
            artifacts,
        })
    }
}

fn artifact_from_checksum_line(line: &str) -> Option<String> {
    let line = line.trim();
    let (_hash, line) = line.split_once(' ')?;
    let (_size, line) = line.split_once(' ')?;
    Some(line.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_buildinfo_sniffglue() {
        let data = r#"-----BEGIN PGP SIGNED MESSAGE-----
Hash: SHA512

Format: 1.0
Source: rust-sniffglue
Binary: librust-sniffglue-dev sniffglue sniffglue-dbgsym
Architecture: amd64
Version: 0.14.0-2
Checksums-Md5:
 6d57946f2f56b1b58d906eb29d6a021f 125404 librust-sniffglue-dev_0.14.0-2_amd64.deb
 20fd28a9824a2c485d2e9b155b0a073c 8995232 sniffglue-dbgsym_0.14.0-2_amd64.deb
 177f9229266ad5eef3fb42fff0c07345 732980 sniffglue_0.14.0-2_amd64.deb
Checksums-Sha1:
 9fdde2501245e7db1eb2eaa104922b1b2b05fe57 125404 librust-sniffglue-dev_0.14.0-2_amd64.deb
 0dc99414756c846aadee63484642958c1ca7ff60 8995232 sniffglue-dbgsym_0.14.0-2_amd64.deb
 d2e4f34a46527effd3375764463d2c1bbe3eeecc 732980 sniffglue_0.14.0-2_amd64.deb
Checksums-Sha256:
 c452054c216359ef44adc9a5d35870d707f47e503051dfcb736f47df17058961 125404 librust-sniffglue-dev_0.14.0-2_amd64.deb
 214817662f43ec4ae0766dd23700a694c45985cb03d28fe82a791a61202e0705 8995232 sniffglue-dbgsym_0.14.0-2_amd64.deb
 448c781a9e594227bc9f0d6c65b8beba2b3add68d3583020de188d4cfa365b40 732980 sniffglue_0.14.0-2_amd64.deb
Build-Origin: Debian
Build-Architecture: amd64
Build-Date: Mon, 06 Dec 2021 21:35:27 +0000
Build-Path: /build/rust-sniffglue-1KDXF6/rust-sniffglue-0.14.0
Installed-Build-Depends:
 autoconf (= 2.71-2),
 automake (= 1:1.16.5-1.1),
 autopoint (= 0.21-4),
 autotools-dev (= 20180224.1+nmu1),
 base-files (= 12),
 base-passwd (= 3.5.52),
 bash (= 5.1-5),
 binutils (= 2.37-10),
 binutils-common (= 2.37-10),
 binutils-x86-64-linux-gnu (= 2.37-10),
 bsdextrautils (= 2.37.2-4),
 bsdutils (= 1:2.37.2-4),
 build-essential (= 12.9),
 bzip2 (= 1.0.8-5),
 cargo (= 0.57.0-3),
 coreutils (= 8.32-4.1),
 cpp (= 4:11.2.0-2),
 cpp-11 (= 11.2.0-12),
 dash (= 0.5.11+git20210903+057cd650a4ed-3),
 debconf (= 1.5.79),
 debhelper (= 13.5.2),
 debianutils (= 5.5-1),
 dh-autoreconf (= 20),
 dh-cargo (= 28),
 dh-strip-nondeterminism (= 1.12.1-1),
 diffutils (= 1:3.7-5),
 dpkg (= 1.20.9),
 dpkg-dev (= 1.20.9),
 dwz (= 0.14-1),
 file (= 1:5.41-2),
 findutils (= 4.8.0-1),
 g++ (= 4:11.2.0-2),
 g++-11 (= 11.2.0-12),
 gcc (= 4:11.2.0-2),
 gcc-11 (= 11.2.0-12),
 gcc-11-base (= 11.2.0-12),
 gettext (= 0.21-4),
 gettext-base (= 0.21-4),
 grep (= 3.7-1),
 groff-base (= 1.22.4-7),
 gzip (= 1.10-4),
 hostname (= 3.23),
 init-system-helpers (= 1.60),
 intltool-debian (= 0.35.0+20060710.5),
 libacl1 (= 2.3.1-1),
 libarchive-zip-perl (= 1.68-1),
 libasan6 (= 11.2.0-12),
 libatomic1 (= 11.2.0-12),
 libattr1 (= 1:2.5.1-1),
 libaudit-common (= 1:3.0.6-1),
 libaudit1 (= 1:3.0.6-1+b1),
 libbinutils (= 2.37-10),
 libblkid1 (= 2.37.2-4),
 libbrotli1 (= 1.0.9-2+b3),
 libbsd0 (= 0.11.3-1),
 libbz2-1.0 (= 1.0.8-5),
 libc-bin (= 2.32-5),
 libc-dev-bin (= 2.32-5),
 libc6 (= 2.32-5),
 libc6-dev (= 2.32-5),
 libcap-ng0 (= 0.7.9-2.2+b1),
 libcap2 (= 1:2.44-1),
 libcc1-0 (= 11.2.0-12),
 libcom-err2 (= 1.46.4-1),
 libcrypt-dev (= 1:4.4.26-1),
 libcrypt1 (= 1:4.4.26-1),
 libctf-nobfd0 (= 2.37-10),
 libctf0 (= 2.37-10),
 libcurl3-gnutls (= 7.79.1-2),
 libdb5.3 (= 5.3.28+dfsg1-0.8),
 libdbus-1-3 (= 1.12.20-3),
 libdbus-1-dev (= 1.12.20-3),
 libdebconfclient0 (= 0.261),
 libdebhelper-perl (= 13.5.2),
 libdpkg-perl (= 1.20.9),
 libedit2 (= 3.1-20210910-1),
 libelf1 (= 0.186-1),
 libexpat1 (= 2.4.1-3),
 libffi8 (= 3.4.2-3),
 libfile-stripnondeterminism-perl (= 1.12.1-1),
 libgcc-11-dev (= 11.2.0-12),
 libgcc-s1 (= 11.2.0-12),
 libgcrypt20 (= 1.9.4-4),
 libgdbm-compat4 (= 1.22-1),
 libgdbm6 (= 1.22-1),
 libgit2-1.1 (= 1.1.0+dfsg.1-4.1),
 libglib2.0-0 (= 2.70.2-1),
 libgmp10 (= 2:6.2.1+dfsg-3),
 libgnutls30 (= 3.7.2-2),
 libgomp1 (= 11.2.0-12),
 libgpg-error0 (= 1.42-3),
 libgssapi-krb5-2 (= 1.18.3-7),
 libhogweed6 (= 3.7.3-1),
 libhttp-parser2.9 (= 2.9.4-4),
 libicu67 (= 67.1-7),
 libidn2-0 (= 2.3.2-2),
 libisl23 (= 0.24-2),
 libitm1 (= 11.2.0-12),
 libk5crypto3 (= 1.18.3-7),
 libkeyutils1 (= 1.6.1-2),
 libkrb5-3 (= 1.18.3-7),
 libkrb5support0 (= 1.18.3-7),
 libldap-2.4-2 (= 2.4.59+dfsg-1),
 libllvm13 (= 1:13.0.0-9+b2),
 liblsan0 (= 11.2.0-12),
 liblz4-1 (= 1.9.3-2),
 liblzma5 (= 5.2.5-2),
 libmagic-mgc (= 1:5.41-2),
 libmagic1 (= 1:5.41-2),
 libmbedcrypto3 (= 2.16.11-0.3),
 libmbedtls12 (= 2.16.11-0.3),
 libmbedx509-0 (= 2.16.11-0.3),
 libmd0 (= 1.0.4-1),
 libmount1 (= 2.37.2-4),
 libmpc3 (= 1.2.1-1),
 libmpdec3 (= 2.5.1-2),
 libmpfr6 (= 4.1.0-3),
 libncursesw6 (= 6.3-1),
 libnettle8 (= 3.7.3-1),
 libnghttp2-14 (= 1.43.0-1),
 libnsl-dev (= 1.3.0-2),
 libnsl2 (= 1.3.0-2),
 libp11-kit0 (= 0.24.0-6),
 libpam-modules (= 1.4.0-10),
 libpam-modules-bin (= 1.4.0-10),
 libpam-runtime (= 1.4.0-10),
 libpam0g (= 1.4.0-10),
 libpcap-dev (= 1.10.1-4),
 libpcap0.8 (= 1.10.1-4),
 libpcap0.8-dev (= 1.10.1-4),
 libpcre2-8-0 (= 10.39-3),
 libpcre3 (= 2:8.39-13),
 libperl5.32 (= 5.32.1-6),
 libpipeline1 (= 1.5.4-1),
 libpsl5 (= 0.21.0-1.2),
 libpython3-stdlib (= 3.9.8-1),
 libpython3.9-minimal (= 3.9.9-1),
 libpython3.9-stdlib (= 3.9.9-1),
 libquadmath0 (= 11.2.0-12),
 libreadline8 (= 8.1-2),
 librtmp1 (= 2.4+20151223.gitfa8646d.1-2+b2),
 librust-aho-corasick+std-dev (= 0.7.10-1),
 librust-aho-corasick-dev (= 0.7.10-1),
 librust-ansi-term-dev (= 0.12.1-1),
 librust-anyhow-dev (= 1.0.44-2),
 librust-arrayvec-dev (= 0.5.1-1),
 librust-atty-dev (= 0.2.14-2),
 librust-autocfg-dev (= 1.0.1-1),
 librust-base64-dev (= 0.13.0-1),
 librust-bitflags-dev (= 1.2.1-1),
 librust-block-buffer-dev (= 0.9.0-4),
 librust-block-padding-dev (= 0.2.1-1),
 librust-bstr+default-dev (= 0.2.17-1),
 librust-bstr+std-dev (= 0.2.17-1),
 librust-bstr+unicode-dev (= 0.2.17-1),
 librust-bstr-dev (= 0.2.17-1),
 librust-byteorder-dev (= 1.4.3-2),
 librust-cc-dev (= 1.0.71-2),
 librust-cfg-if-0.1-dev (= 0.1.10-2),
 librust-cfg-if-dev (= 1.0.0-1),
 librust-clap+color-dev (= 2.33.3-1),
 librust-clap+default-dev (= 2.33.3-1),
 librust-clap+strsim-dev (= 2.33.3-1),
 librust-clap-dev (= 2.33.3-1),
 librust-compiler-builtins-dev (= 0.1.52-1),
 librust-cpuid-bool-dev (= 0.1.2-4),
 librust-dhcp4r-dev (= 0.2.0-1),
 librust-digest-dev (= 0.9.0-1),
 librust-dirs-next-dev (= 2.0.0-1),
 librust-dirs-sys-next-dev (= 0.1.1-1+b1),
 librust-dns-parser-dev (= 0.8.0-1+b1),
 librust-enum-primitive-derive-dev (= 0.1.2-2),
 librust-enum-primitive-dev (= 0.1.1-1+b1),
 librust-env-logger+default-dev (= 0.9.0-1),
 librust-env-logger-dev (= 0.9.0-1),
 librust-generic-array-dev (= 0.14.4-1),
 librust-getrandom-dev (= 0.1.13-4),
 librust-heck-dev (= 0.3.3-1),
 librust-humantime-dev (= 2.1.0-1),
 librust-itoa-dev (= 0.4.3-1),
 librust-lazy-static-dev (= 1.4.0-1),
 librust-lexical-core+default-dev (= 0.4.8-3),
 librust-lexical-core-dev (= 0.4.8-3),
 librust-libc-dev (= 0.2.103-1),
 librust-libm-dev (= 0.2.1-3),
 librust-log-dev (= 0.4.11-2),
 librust-memchr-dev (= 2.4.1-1),
 librust-memoffset-dev (= 0.6.4-1),
 librust-nix-dev (= 0.23.0-1),
 librust-nom+default-dev (= 5.0.1-4),
 librust-nom+lexical-dev (= 5.0.1-4),
 librust-nom+std-dev (= 5.0.1-4),
 librust-nom-dev (= 5.0.1-4),
 librust-num-cpus-dev (= 1.13.0-1),
 librust-num-traits-dev (= 0.2.14-1),
 librust-opaque-debug-dev (= 0.3.0-1),
 librust-pcap-sys-dev (= 0.1.3-2),
 librust-phf+std-dev (= 0.8.0-2),
 librust-phf-codegen-dev (= 0.8.0-1),
 librust-phf-dev (= 0.8.0-2),
 librust-phf-generator-dev (= 0.8.0-1),
 librust-phf-shared-dev (= 0.8.0-1),
 librust-pkg-config-dev (= 0.3.21-1),
 librust-pktparse+serde-dev (= 0.5.0-1),
 librust-pktparse-dev (= 0.5.0-1),
 librust-ppv-lite86-dev (= 0.2.6-2),
 librust-proc-macro-error-attr-dev (= 1.0.3-1),
 librust-proc-macro-error-dev (= 1.0.3-1),
 librust-proc-macro2-dev (= 1.0.28-1),
 librust-quick-error-dev (= 1.2.3-1),
 librust-quote+proc-macro-dev (= 1.0.9-1),
 librust-quote-dev (= 1.0.9-1),
 librust-rand+alloc-dev (= 0.7.3-3),
 librust-rand+getrandom-dev (= 0.7.3-3),
 librust-rand+rand-pcg-dev (= 0.7.3-3),
 librust-rand+std-dev (= 0.7.3-3),
 librust-rand-chacha+std-dev (= 0.2.2-1),
 librust-rand-chacha-dev (= 0.2.2-1),
 librust-rand-core+getrandom-dev (= 0.5.1-1),
 librust-rand-core+std-dev (= 0.5.1-1),
 librust-rand-core-dev (= 0.5.1-1),
 librust-rand-dev (= 0.7.3-3),
 librust-rand-hc-dev (= 0.2.0-1+b1),
 librust-rand-pcg-dev (= 0.2.1-1),
 librust-redox-syscall-dev (= 0.1.57-2),
 librust-reduce-dev (= 0.1.1-1+b1),
 librust-regex+perf-dev (= 1.5.4-1),
 librust-regex+perf-literal-dev (= 1.5.4-1),
 librust-regex-automata-dev (= 0.1.8-2),
 librust-regex-dev (= 1.5.4-1),
 librust-regex-syntax-dev (= 0.6.25-1),
 librust-rustc-std-workspace-core-dev (= 1.0.0-1+b1),
 librust-rustc-version-dev (= 0.4.0-1),
 librust-rusticata-macros-dev (= 2.0.4-1),
 librust-ryu-dev (= 1.0.2-1),
 librust-seccomp-sys-dev (= 0.1.3-1),
 librust-semver-dev (= 1.0.4-1),
 librust-serde+derive-dev (= 1.0.130-2),
 librust-serde-derive-dev (= 1.0.130-1),
 librust-serde-dev (= 1.0.130-2),
 librust-serde-json-dev (= 1.0.41-1),
 librust-sha2-asm-dev (= 0.5.1-3+b1),
 librust-sha2-dev (= 0.9.2-2),
 librust-siphasher-dev (= 0.3.1-1),
 librust-static-assertions-dev (= 1.1.0-1),
 librust-strsim-dev (= 0.9.3-1),
 librust-structopt+default-dev (= 0.3.20-1),
 librust-structopt-derive-dev (= 0.4.13-1),
 librust-structopt-dev (= 0.3.20-1),
 librust-strum-dev (= 0.19.2-1),
 librust-strum-macros-dev (= 0.19.2-1),
 librust-syn+default-dev (= 1.0.76-1),
 librust-syn+printing-dev (= 1.0.76-1),
 librust-syn+proc-macro-dev (= 1.0.76-1),
 librust-syn-dev (= 1.0.76-1),
 librust-syn-mid-dev (= 0.5.0-2),
 librust-syscallz-dev (= 0.15.0-1),
 librust-termcolor-dev (= 1.1.0-1),
 librust-textwrap-dev (= 0.11.0-1+b1),
 librust-time-dev (= 0.1.42-1),
 librust-tls-parser-dev (= 0.9.2-3),
 librust-toml-dev (= 0.5.8-1),
 librust-typenum-dev (= 1.12.0-1),
 librust-unicode-segmentation-dev (= 1.6.0-1),
 librust-unicode-width-dev (= 0.1.8-1),
 librust-unicode-xid-dev (= 0.2.0-1),
 librust-users+default-dev (= 0.11.0-1),
 librust-users-dev (= 0.11.0-1),
 librust-vec-map-dev (= 0.8.1-2+b1),
 librust-version-check-dev (= 0.9.2-1),
 librust-winapi-dev (= 0.3.9-1),
 librust-winapi-i686-pc-windows-gnu-dev (= 0.4.0-1+b1),
 librust-winapi-util-dev (= 0.1.5-1),
 librust-winapi-x86-64-pc-windows-gnu-dev (= 0.4.0-1+b1),
 libsasl2-2 (= 2.1.27+dfsg2-2),
 libsasl2-modules-db (= 2.1.27+dfsg2-2),
 libseccomp-dev (= 2.5.3-2),
 libseccomp2 (= 2.5.3-2),
 libselinux1 (= 3.3-1+b1),
 libsigsegv2 (= 2.13-1),
 libsmartcols1 (= 2.37.2-4),
 libsqlite3-0 (= 3.36.0-2),
 libssh2-1 (= 1.10.0-2),
 libssl1.1 (= 1.1.1l-1),
 libstd-rust-1.56 (= 1.56.0+dfsg1-2),
 libstd-rust-dev (= 1.56.0+dfsg1-2),
 libstdc++-11-dev (= 11.2.0-12),
 libstdc++6 (= 11.2.0-12),
 libsub-override-perl (= 0.09-2),
 libsystemd0 (= 249.7-1),
 libtasn1-6 (= 4.18.0-4),
 libtinfo6 (= 6.3-1),
 libtirpc-common (= 1.3.2-2),
 libtirpc-dev (= 1.3.2-2),
 libtirpc3 (= 1.3.2-2),
 libtool (= 2.4.6-15),
 libtsan0 (= 11.2.0-12),
 libubsan1 (= 11.2.0-12),
 libuchardet0 (= 0.0.7-1),
 libudev1 (= 249.7-1),
 libunistring2 (= 0.9.10-6),
 libuuid1 (= 2.37.2-4),
 libxml2 (= 2.9.12+dfsg-5+b1),
 libz3-4 (= 4.8.12-1+b1),
 libzstd1 (= 1.4.8+dfsg-3),
 linux-libc-dev (= 5.15.5-1),
 login (= 1:4.8.1-2),
 lsb-base (= 11.1.0),
 m4 (= 1.4.18-5),
 make (= 4.3-4.1),
 man-db (= 2.9.4-2),
 mawk (= 1.3.4.20200120-2),
 media-types (= 4.0.0),
 ncurses-base (= 6.3-1),
 ncurses-bin (= 6.3-1),
 patch (= 2.7.6-7),
 perl (= 5.32.1-6),
 perl-base (= 5.32.1-6),
 perl-modules-5.32 (= 5.32.1-6),
 pkg-config (= 0.29.2-1),
 po-debconf (= 1.0.21+nmu1),
 python3 (= 3.9.8-1),
 python3-minimal (= 3.9.8-1),
 python3.9 (= 3.9.9-1),
 python3.9-minimal (= 3.9.9-1),
 readline-common (= 8.1-2),
 rpcsvc-proto (= 1.4.2-4),
 rustc (= 1.56.0+dfsg1-2),
 sed (= 4.8-1),
 sensible-utils (= 0.0.17),
 sysvinit-utils (= 3.00-1),
 tar (= 1.34+dfsg-1),
 tzdata (= 2021e-1),
 util-linux (= 2.37.2-4),
 xz-utils (= 5.2.5-2),
 zlib1g (= 1:1.2.11.dfsg-2)
Environment:
 DEB_BUILD_OPTIONS="parallel=4"
 LC_ALL="C.UTF-8"
 LC_COLLATE="C.UTF-8"
 SOURCE_DATE_EPOCH="1638820316"

-----BEGIN PGP SIGNATURE-----

iQIzBAEBCgAdFiEE8DPOGMaQHbqWZUKpt/b36/s0kbEFAmGugiIACgkQt/b36/s0
kbG/Dg//fBd1UMRbxCY5r+/aa3ptKzO3Lrh9BC4WMlkqrmfCLUJbKoGef6Rqi8Dj
TPWQ7JVG1Ji8T7OAL7awHZssAic9xro6+lPTA5Yr8K+bfBlMjhvHpYw1XkWMddMW
ASTAP+TCJZ1EoCjP2DafYIlEfLbJPp0DkCFrbtvzhZc07jJ3zAeHXahW6D+yI194
GYuQ1XK0TIJ5oHf/c9GAGiiLMcjfaQhqljT8Rm5YmEl8P27ctprucrOAvI9hk3Ym
WaGO7c+wOWo1Tce3t3ds2pRgqw7shZRES8Ei6yE62Dr6AC5kBF5jD/D4JXnTddbz
JMmppowi+twUDeCMSN5u+46H0lCBsPnl+UOl8zALyYruVDroBvepKPAOvJcHTBp7
8B5m+p7DFZdQZXSjMWud94gg7T6CSYitr6Am4/851b8Jc13fVV7imrLXoeLU2GBL
CGKbBcLpte+YNEIV3EosYOpV610XpIoZkEmoVgx3bwcaj1kn9q2IJsIH7vDl8M/u
neDipbtq1yg9jcxF9MEmpsMxaPwnm+h+8c+yoyuv9MfKz4OcaCngCPFF/CVFTgHs
rLtioF1ivcIxRqx/8541vxERmntZ8Ud0bd1a5DJVGoMAh6AFHjMlqNPJNC3pUYdv
8fKIaaQEB+SEa7PPGI0J6j9yqlluJZbMDHuPgU52Bq29Qm/ZOes=
=eBQQ
-----END PGP SIGNATURE-----
"#;
        let buildinfo = Buildinfo::from_str(data).unwrap();
        let mut artifacts = HashSet::new();
        artifacts.insert("librust-sniffglue-dev_0.14.0-2_amd64.deb".to_string());
        artifacts.insert("sniffglue-dbgsym_0.14.0-2_amd64.deb".to_string());
        artifacts.insert("sniffglue_0.14.0-2_amd64.deb".to_string());
        assert_eq!(
            buildinfo,
            Buildinfo {
                source: "rust-sniffglue".to_string(),
                version: "0.14.0-2".to_string(),
                artifacts,
            }
        );
    }

    #[test]
    fn parse_buildinfo_courier() {
        let data = r#"-----BEGIN PGP SIGNED MESSAGE-----
Hash: SHA512

Format: 1.0
Source: courier (1.0.16-3)
Binary: courier-base courier-base-dbgsym courier-faxmail courier-imap courier-imap-dbgsym courier-ldap courier-ldap-dbgsym courier-mlm courier-mlm-dbgsym courier-mta courier-mta-dbgsym courier-pcp courier-pcp-dbgsym courier-pop courier-pop-dbgsym courier-webadmin courier-webadmin-dbgsym sqwebmail sqwebmail-dbgsym
Architecture: amd64
Version: 1.0.16-3+b1
Binary-Only-Changes:
 courier (1.0.16-3+b1) sid; urgency=low, binary-only=yes
 .
   * Binary-only non-maintainer upload for amd64; no source changes.
   * Rebuild against libidn12
 .
  -- amd64 / i386 Build Daemon (x86-ubc-01) <buildd_amd64-x86-ubc-01@buildd.debian.org>  Sun, 22 Aug 2021 22:12:19 +0000
Checksums-Md5:
 224dacec94be1775639ef61ff01efea9 459456 courier-base-dbgsym_1.0.16-3+b1_amd64.deb
 06d0be1d5211f0d425803633806663a5 328148 courier-base_1.0.16-3+b1_amd64.deb
 24f9a41c6380ea2ca7559ce9e6f6f54a 137492 courier-faxmail_1.0.16-3+b1_amd64.deb
 56f61b3dd8a9b855a688bf10fe982e9a 504672 courier-imap-dbgsym_5.0.13+1.0.16-3+b1_amd64.deb
 f8eb4b0c42f191581189b2aee4c31793 277952 courier-imap_5.0.13+1.0.16-3+b1_amd64.deb
 56ab0d8f58360e7f9e9cdd1e550ef4aa 31896 courier-ldap-dbgsym_1.0.16-3+b1_amd64.deb
 fda5c8d7f98633778de1beb7197d257c 142284 courier-ldap_1.0.16-3+b1_amd64.deb
 786868f86bedc808f60f185b7e1ca2a1 2975168 courier-mlm-dbgsym_1.0.16-3+b1_amd64.deb
 4f8cc681fee4364d1a89e118c16bb5a7 390264 courier-mlm_1.0.16-3+b1_amd64.deb
 bab1bb694fe552a42472b2b0c9cf39bb 3233060 courier-mta-dbgsym_1.0.16-3+b1_amd64.deb
 788546e04c005d285cc084c1dea6b2ab 634364 courier-mta_1.0.16-3+b1_amd64.deb
 1e94cc602065e1df6b0a57212da60ffc 148160 courier-pcp-dbgsym_1.0.16-3+b1_amd64.deb
 16d6b5a9f33b6da3e8cbe0fb52105aa5 169328 courier-pcp_1.0.16-3+b1_amd64.deb
 97c64a4c770f9b613ac9eca381ddf162 134468 courier-pop-dbgsym_1.0.16-3+b1_amd64.deb
 97bbb2d0a27f34d1b48482cfd003b081 179872 courier-pop_1.0.16-3+b1_amd64.deb
 04031603ee4c00f5d972bd7b61c89bc3 3992 courier-webadmin-dbgsym_1.0.16-3+b1_amd64.deb
 146605f7e774c98d94da493cc3c19c4e 147916 courier-webadmin_1.0.16-3+b1_amd64.deb
 594fb250bfd24cd9b570cd8881ab88df 1008928 sqwebmail-dbgsym_6.0.5+1.0.16-3+b1_amd64.deb
 5fa070104394a53254b244b16685508e 496944 sqwebmail_6.0.5+1.0.16-3+b1_amd64.deb
Checksums-Sha1:
 5c9481e660beb92d12a315e1eee3174ed0024bcf 459456 courier-base-dbgsym_1.0.16-3+b1_amd64.deb
 47f73e25e38bcc966bd9db4f01b20a89cacdac38 328148 courier-base_1.0.16-3+b1_amd64.deb
 a7df6d382b73c34bb22d14cca2c6228d5a4ef0e0 137492 courier-faxmail_1.0.16-3+b1_amd64.deb
 da99a63aa0653506414a167b93cd55b801b1a45c 504672 courier-imap-dbgsym_5.0.13+1.0.16-3+b1_amd64.deb
 e9ac59293bcad55ae1a8e884510633f0b2387fc7 277952 courier-imap_5.0.13+1.0.16-3+b1_amd64.deb
 cae9c8e7ec94c448844a29531e5c6ad0a3dc0527 31896 courier-ldap-dbgsym_1.0.16-3+b1_amd64.deb
 1c4550c6dcca151a94b5bd23e859d5ec325d84f9 142284 courier-ldap_1.0.16-3+b1_amd64.deb
 5a0f79081975dc7fd91b0a84664b6a32639ba509 2975168 courier-mlm-dbgsym_1.0.16-3+b1_amd64.deb
 1faa6b2ba5e02e349cbb9bbb0f5c18bd196722da 390264 courier-mlm_1.0.16-3+b1_amd64.deb
 b9a1eb39e916317db01a9b5df760066736b8db51 3233060 courier-mta-dbgsym_1.0.16-3+b1_amd64.deb
 0e1b9802303b48bab4b2f0377c40b18b0464991a 634364 courier-mta_1.0.16-3+b1_amd64.deb
 38cfe238d668a31a24356af200380dd96160c38e 148160 courier-pcp-dbgsym_1.0.16-3+b1_amd64.deb
 31f33ecfd6b8f59a1826c9b695bdd2f3bf37bdd5 169328 courier-pcp_1.0.16-3+b1_amd64.deb
 f701fe7cf62ca401161a92353bebf9d3892611e7 134468 courier-pop-dbgsym_1.0.16-3+b1_amd64.deb
 2ab1cb53c878452b40d9b266bd41b5fbe5a23c87 179872 courier-pop_1.0.16-3+b1_amd64.deb
 f0a7436e84b642562f50659aa8c0b772a934077f 3992 courier-webadmin-dbgsym_1.0.16-3+b1_amd64.deb
 a00ddf85da6ad81ba1db25317779165f273bd6c4 147916 courier-webadmin_1.0.16-3+b1_amd64.deb
 14288aedb222350809795ca82502c9e36078b82b 1008928 sqwebmail-dbgsym_6.0.5+1.0.16-3+b1_amd64.deb
 34d298eaa1065ad5cff91bdec577cb7a91142654 496944 sqwebmail_6.0.5+1.0.16-3+b1_amd64.deb
Checksums-Sha256:
 786673efeab0460e94d1a1e60f36aa8732bb4535ae85746dee471a6db49fb676 459456 courier-base-dbgsym_1.0.16-3+b1_amd64.deb
 688b7c11b8ec92514929d37e207681e4b9ac754db9e8cf0ab0632374433eed7e 328148 courier-base_1.0.16-3+b1_amd64.deb
 6cb78e731f845dd98ab792c43fc01a6dc3416140b08d2db00e7415eff5973527 137492 courier-faxmail_1.0.16-3+b1_amd64.deb
 c90cc58b7c957b90120b606f39422315bca05293defacc1d4cf42ce4aa178128 504672 courier-imap-dbgsym_5.0.13+1.0.16-3+b1_amd64.deb
 67acfd8593f6c0a12a2681906e43dfe2872a694bd74e5438726646ec0e2af0a6 277952 courier-imap_5.0.13+1.0.16-3+b1_amd64.deb
 a146803c918d1160ca4681bd62b3bc61e14f15bed0cd483ae66916e7b279c57a 31896 courier-ldap-dbgsym_1.0.16-3+b1_amd64.deb
 733c1e1f620b416fb107e115fc8fa3cbe511d1e47ce1e171e1ffb8b5b1cecd05 142284 courier-ldap_1.0.16-3+b1_amd64.deb
 aaf96264630a4526c7aff2502a8015c0d409ca6e772213d7b9014cce3f1ecb63 2975168 courier-mlm-dbgsym_1.0.16-3+b1_amd64.deb
 209018400fd2dfa4caf4ede13a8865a110a9dec1c387e8eb9e45e2cd0550b771 390264 courier-mlm_1.0.16-3+b1_amd64.deb
 5a513509987478410e297f99be9d0d644f999928c7bc9ad1e854ad25473eeee0 3233060 courier-mta-dbgsym_1.0.16-3+b1_amd64.deb
 a05a38e8aa0986067b40d1f82dbf59732c6f4697ea91d8717ef0ea88b388ae6a 634364 courier-mta_1.0.16-3+b1_amd64.deb
 254defd40412e5f6a09d4ca4784b07255f617d2a1bb2ed1f7deac6d67a1b9771 148160 courier-pcp-dbgsym_1.0.16-3+b1_amd64.deb
 7b118060d17983ff0a070825819a5b6ac46bad0262288e5f63c7bd0baefcf72d 169328 courier-pcp_1.0.16-3+b1_amd64.deb
 655cd79a41636291ed2eb4be416828d14d7d44c55cebeb139ddce65d33390dcf 134468 courier-pop-dbgsym_1.0.16-3+b1_amd64.deb
 efc865b63f19efda4feb15917789b9390667d973ac18d7aa6e641290a7ba8461 179872 courier-pop_1.0.16-3+b1_amd64.deb
 e7f7abf143674c7ac1fe5a6c0c21ac2d9505c3ade37625e7623a6d13ccc6e45f 3992 courier-webadmin-dbgsym_1.0.16-3+b1_amd64.deb
 bc65ecb8eac668c0c5fe18c6784217dd67541d2347899e0096b4b2f9f2ab0059 147916 courier-webadmin_1.0.16-3+b1_amd64.deb
 e48e7fb6a38b6dd2057f61db875368cf5d175bc5b012c5b88eba8f4c15b4321e 1008928 sqwebmail-dbgsym_6.0.5+1.0.16-3+b1_amd64.deb
 51ebf109a5257b34a521a68967db5b328d2301ad9e7585ccf797dfb549ed5e6e 496944 sqwebmail_6.0.5+1.0.16-3+b1_amd64.deb
Build-Origin: Debian
Build-Architecture: amd64
Build-Date: Sun, 22 Aug 2021 22:24:21 +0000
Build-Path: /build/courier-DEOrho/courier-1.0.16
Installed-Build-Depends:
 adduser (= 3.118),
 apache2-dev (= 2.4.48-4),
 autoconf (= 2.69-14),
 automake (= 1:1.16.4-1),
 autopoint (= 0.21-4),
 autotools-dev (= 20180224.1+nmu1),
 base-files (= 11.1),
 base-passwd (= 3.5.51),
 bash (= 5.1-3+b1),
 binutils (= 2.37-4),
 binutils-common (= 2.37-4),
 binutils-x86-64-linux-gnu (= 2.37-4),
 bsdextrautils (= 2.37.2-1),
 bsdutils (= 1:2.36.1-8),
 build-essential (= 12.9),
 bzip2 (= 1.0.8-4),
 coreutils (= 8.32-4+b1),
 courier-authlib (= 0.71.1-2),
 courier-authlib-dev (= 0.71.1-2),
 cpp (= 4:10.2.1-1),
 cpp-10 (= 10.2.1-6),
 cron (= 3.0pl1-137),
 dash (= 0.5.11+git20210120+802ebd4-1),
 debconf (= 1.5.77),
 debhelper (= 13.4.1),
 debianutils (= 5.1-1),
 default-libmysqlclient-dev (= 1.0.7),
 dh-autoreconf (= 20),
 dh-exec (= 0.23.4),
 dh-strip-nondeterminism (= 1.12.0-1),
 diffutils (= 1:3.7-5),
 dirmngr (= 2.2.27-2),
 dpkg (= 1.20.9),
 dpkg-dev (= 1.20.9),
 dwz (= 0.14-1),
 expect (= 5.45.4-2+b1),
 file (= 1:5.39-3),
 findutils (= 4.8.0-1),
 fontconfig-config (= 2.13.1-4.2),
 fonts-urw-base35 (= 20200910-1),
 g++ (= 4:10.2.1-1),
 g++-10 (= 10.2.1-6),
 gamin (= 0.1.10-6),
 gcc (= 4:10.2.1-1),
 gcc-10 (= 10.2.1-6),
 gcc-10-base (= 10.2.1-6),
 gcc-11-base (= 11.2.0-2),
 gettext (= 0.21-4),
 gettext-base (= 0.21-4),
 ghostscript (= 9.53.3~dfsg-7+b1),
 gnupg (= 2.2.27-2),
 gnupg-l10n (= 2.2.27-2),
 gnupg-utils (= 2.2.27-2),
 gnupg2 (= 2.2.27-2),
 gnutls-bin (= 3.7.1-5),
 gpg (= 2.2.27-2),
 gpg-agent (= 2.2.27-2),
 gpg-wks-client (= 2.2.27-2),
 gpg-wks-server (= 2.2.27-2),
 gpgconf (= 2.2.27-2),
 gpgsm (= 2.2.27-2),
 gpgv (= 2.2.27-2),
 grep (= 3.6-1),
 groff-base (= 1.22.4-6),
 gzip (= 1.10-4),
 hostname (= 3.23),
 init-system-helpers (= 1.60),
 intltool-debian (= 0.35.0+20060710.5),
 libacl1 (= 2.2.53-10),
 libapr1 (= 1.7.0-6),
 libapr1-dev (= 1.7.0-6),
 libaprutil1 (= 1.6.1-5),
 libaprutil1-dev (= 1.6.1-5),
 libarchive-zip-perl (= 1.68-1),
 libasan6 (= 11.2.0-2),
 libassuan0 (= 2.5.5-1),
 libatomic1 (= 11.2.0-2),
 libattr1 (= 1:2.4.48-6),
 libaudit-common (= 1:3.0.5-1),
 libaudit1 (= 1:3.0.5-1),
 libavahi-client3 (= 0.8-5),
 libavahi-common-data (= 0.8-5),
 libavahi-common3 (= 0.8-5),
 libbinutils (= 2.37-4),
 libblkid1 (= 2.36.1-8),
 libbrotli1 (= 1.0.9-2+b2),
 libbz2-1.0 (= 1.0.8-4),
 libc-bin (= 2.31-16),
 libc-dev-bin (= 2.31-16),
 libc6 (= 2.31-16),
 libc6-dev (= 2.31-16),
 libcap-ng0 (= 0.7.9-2.2+b1),
 libcc1-0 (= 11.2.0-2),
 libcom-err2 (= 1.46.2-2),
 libcourier-unicode-dev (= 2.1.2-2),
 libcourier-unicode4 (= 2.1.2-2),
 libcrypt-dev (= 1:4.4.25-1),
 libcrypt1 (= 1:4.4.25-1),
 libctf-nobfd0 (= 2.37-4),
 libctf0 (= 2.37-4),
 libcups2 (= 2.3.3op2-3+deb11u1),
 libdb5.3 (= 5.3.28+dfsg1-0.8),
 libdbus-1-3 (= 1.12.20-2),
 libdebconfclient0 (= 0.260),
 libdebhelper-perl (= 13.4.1),
 libdeflate0 (= 1.7-2),
 libdpkg-perl (= 1.20.9),
 libelf1 (= 0.185-2),
 libevent-2.1-7 (= 2.1.12-stable-1),
 libexpat1 (= 2.2.10-2),
 libexpat1-dev (= 2.2.10-2),
 libffi7 (= 3.3-6),
 libfile-stripnondeterminism-perl (= 1.12.0-1),
 libfontconfig1 (= 2.13.1-4.2),
 libfreetype6 (= 2.10.4+dfsg-1),
 libgamin-dev (= 0.1.10-6),
 libgamin0 (= 0.1.10-6),
 libgcc-10-dev (= 10.2.1-6),
 libgcc-s1 (= 11.2.0-2),
 libgcrypt20 (= 1.8.7-6),
 libgcrypt20-dev (= 1.8.7-6),
 libgdbm-compat4 (= 1.19-2),
 libgdbm-dev (= 1.19-2),
 libgdbm6 (= 1.19-2),
 libglib2.0-0 (= 2.68.4-1),
 libgmp-dev (= 2:6.2.1+dfsg-1),
 libgmp10 (= 2:6.2.1+dfsg-1),
 libgmpxx4ldbl (= 2:6.2.1+dfsg-1),
 libgnutls-dane0 (= 3.7.1-5),
 libgnutls-openssl27 (= 3.7.1-5),
 libgnutls28-dev (= 3.7.1-5),
 libgnutls30 (= 3.7.1-5),
 libgnutlsxx28 (= 3.7.1-5),
 libgomp1 (= 11.2.0-2),
 libgpg-error-dev (= 1.42-2),
 libgpg-error0 (= 1.42-2),
 libgs9 (= 9.53.3~dfsg-7+b1),
 libgs9-common (= 9.53.3~dfsg-7),
 libgssapi-krb5-2 (= 1.18.3-6),
 libhogweed6 (= 3.7.3-1),
 libicu67 (= 67.1-7),
 libidn-dev (= 1.38-3),
 libidn11-dev (= 1.38-3),
 libidn12 (= 1.38-3),
 libidn2-0 (= 2.3.2-2),
 libidn2-dev (= 2.3.2-2),
 libijs-0.35 (= 0.35-15),
 libisl23 (= 0.23-1),
 libitm1 (= 11.2.0-2),
 libjbig0 (= 2.1-3.1+b2),
 libjbig2dec0 (= 0.19-3),
 libjpeg62-turbo (= 1:2.0.6-4),
 libk5crypto3 (= 1.18.3-6),
 libkeyutils1 (= 1.6.1-2),
 libkrb5-3 (= 1.18.3-6),
 libkrb5support0 (= 1.18.3-6),
 libksba8 (= 1.5.0-3),
 liblcms2-2 (= 2.12~rc1-2),
 libldap-2.4-2 (= 2.4.57+dfsg-3),
 libldap2-dev (= 2.4.57+dfsg-3),
 liblsan0 (= 11.2.0-2),
 libltdl7 (= 2.4.6-15),
 liblz4-1 (= 1.9.3-2),
 liblzma5 (= 5.2.5-2),
 libmagic-mgc (= 1:5.39-3),
 libmagic1 (= 1:5.39-3),
 libmariadb-dev (= 1:10.5.12-1),
 libmariadb-dev-compat (= 1:10.5.12-1),
 libmariadb3 (= 1:10.5.12-1),
 libmount1 (= 2.36.1-8),
 libmpc3 (= 1.2.0-1),
 libmpdec3 (= 2.5.1-2),
 libmpfr6 (= 4.1.0-3),
 libncurses6 (= 6.2+20201114-2),
 libncursesw6 (= 6.2+20201114-2),
 libnetpbm10 (= 2:10.0-15.4),
 libnettle8 (= 3.7.3-1),
 libnpth0 (= 1.6-3),
 libnsl-dev (= 1.3.0-2),
 libnsl2 (= 1.3.0-2),
 libopenjp2-7 (= 2.4.0-3),
 libopts25 (= 1:5.18.16-4),
 libp11-kit-dev (= 0.23.22-1),
 libp11-kit0 (= 0.23.22-1),
 libpam-modules (= 1.4.0-9),
 libpam-modules-bin (= 1.4.0-9),
 libpam-runtime (= 1.4.0-9),
 libpam0g (= 1.4.0-9),
 libpam0g-dev (= 1.4.0-9),
 libpaper1 (= 1.1.28+b1),
 libpcre16-3 (= 2:8.39-13),
 libpcre2-8-0 (= 10.36-2),
 libpcre3 (= 2:8.39-13),
 libpcre3-dev (= 2:8.39-13),
 libpcre32-3 (= 2:8.39-13),
 libpcrecpp0v5 (= 2:8.39-13),
 libperl-dev (= 5.32.1-5),
 libperl5.32 (= 5.32.1-5),
 libpipeline1 (= 1.5.3-1),
 libpng16-16 (= 1.6.37-3),
 libpopt0 (= 1.18-2),
 libpq-dev (= 13.4-1),
 libpq5 (= 13.4-1),
 libprocps8 (= 2:3.3.17-5),
 libpsl5 (= 0.21.0-1.2),
 libpython3-stdlib (= 3.9.2-3),
 libpython3.9-minimal (= 3.9.2-1),
 libpython3.9-stdlib (= 3.9.2-1),
 libquadmath0 (= 11.2.0-2),
 libreadline8 (= 8.1-2),
 libsasl2-2 (= 2.1.27+dfsg-2.1),
 libsasl2-dev (= 2.1.27+dfsg-2.1),
 libsasl2-modules-db (= 2.1.27+dfsg-2.1),
 libsctp-dev (= 1.0.19+dfsg-1),
 libsctp1 (= 1.0.19+dfsg-1),
 libseccomp2 (= 2.5.1-1),
 libselinux1 (= 3.1-3),
 libsemanage-common (= 3.1-1),
 libsemanage1 (= 3.1-1+b2),
 libsepol1 (= 3.1-1),
 libsigsegv2 (= 2.13-1),
 libsmartcols1 (= 2.36.1-8),
 libsqlite3-0 (= 3.34.1-3),
 libssl-dev (= 1.1.1k-1),
 libssl1.1 (= 1.1.1k-1),
 libstdc++-10-dev (= 10.2.1-6),
 libstdc++6 (= 11.2.0-2),
 libsub-override-perl (= 0.09-2),
 libsystemd0 (= 247.9-1),
 libtasn1-6 (= 4.16.0-2),
 libtasn1-6-dev (= 4.16.0-2),
 libtcl8.6 (= 8.6.11+dfsg-1),
 libtiff5 (= 4.2.0-1),
 libtinfo6 (= 6.2+20201114-2),
 libtirpc-common (= 1.3.2-2),
 libtirpc-dev (= 1.3.2-2),
 libtirpc3 (= 1.3.2-2),
 libtool (= 2.4.6-15),
 libtool-bin (= 2.4.6-15),
 libtsan0 (= 11.2.0-2),
 libubsan1 (= 11.2.0-2),
 libuchardet0 (= 0.0.7-1),
 libudev1 (= 247.9-1),
 libunbound8 (= 1.13.1-1),
 libunistring2 (= 0.9.10-6),
 libuuid1 (= 2.37.2-1),
 libwebp6 (= 0.6.1-2.1),
 libxml2 (= 2.9.10+dfsg-6.7),
 libzstd1 (= 1.4.8+dfsg-2.1),
 linux-libc-dev (= 5.10.46-4),
 login (= 1:4.8.1-1),
 logrotate (= 3.18.1-2),
 lsb-base (= 11.1.0),
 m4 (= 1.4.18-5),
 mailcap (= 3.70),
 make (= 4.3-4.1),
 man-db (= 2.9.4-2),
 mariadb-common (= 1:10.5.12-1),
 mawk (= 1.3.4.20200120-2),
 media-types (= 4.0.0),
 mgetty (= 1.2.1-1.1),
 mgetty-fax (= 1.2.1-1.1),
 mime-support (= 3.66),
 mysql-common (= 5.8+1.0.7),
 ncurses-base (= 6.2+20201114-2),
 ncurses-bin (= 6.2+20201114-2),
 netpbm (= 2:10.0-15.4),
 nettle-dev (= 3.7.3-1),
 openssl (= 1.1.1k-1),
 passwd (= 1:4.8.1-1),
 patch (= 2.7.6-7),
 perl (= 5.32.1-5),
 perl-base (= 5.32.1-5),
 perl-modules-5.32 (= 5.32.1-5),
 pinentry-curses (= 1.1.0-4),
 pkg-config (= 0.29.2-1),
 po-debconf (= 1.0.21+nmu1),
 poppler-data (= 0.4.10-1),
 procps (= 2:3.3.17-5),
 python3 (= 3.9.2-3),
 python3-minimal (= 3.9.2-3),
 python3.9 (= 3.9.2-1),
 python3.9-minimal (= 3.9.2-1),
 readline-common (= 8.1-2),
 rpcsvc-proto (= 1.4.2-3),
 sed (= 4.7-1),
 sensible-utils (= 0.0.14),
 sysvinit-utils (= 2.96-7),
 tar (= 1.34+dfsg-1),
 tcl-expect (= 5.45.4-2+b1),
 tcl8.6 (= 8.6.11+dfsg-1),
 tzdata (= 2021a-1),
 ucf (= 3.0043),
 util-linux (= 2.36.1-8),
 uuid-dev (= 2.37.2-1),
 wget (= 1.21-1+b1),
 xz-utils (= 5.2.5-2),
 zlib1g (= 1:1.2.11.dfsg-2),
 zlib1g-dev (= 1:1.2.11.dfsg-2)
Environment:
 DEB_BUILD_OPTIONS="parallel=4"
 LC_ALL="C.UTF-8"
 LC_COLLATE="C.UTF-8"
 SOURCE_DATE_EPOCH="1629670339"

-----BEGIN PGP SIGNATURE-----

iQIzBAEBCgAdFiEEVvgiDm0iTi84B8TiOTy2rP5qAaMFAmEizpwACgkQOTy2rP5q
AaPyYw/+Kvr81SRwW53qD7vKIeCjWfwOYXbEghmjGuN+thTdSyA6jAt7dvudtA+J
x6gAqb2UUhN8fVuU2pyUTblzDukVlTq1+RLGx8QkhCUJNtwwNmZHf/yneEJOr6Q9
EfGMRw6VInJY8W0k6/mL82KLXUKqsb6UTI/H4sCpjYo9A9uPaXzT+XymQGnycFne
6BJcK9pdCuKkvAAOBCV/m8UUEIMdZJ2VM9EweCm5fb9+yoKLVKCx4eejrA2qKVjA
uwthkk+U1I0qlVVYXbgjUjm7zVIqZYiFeF8jls1BoTdNAuwOYKKw5bHP3vK1M850
nIj6ByadeWJgUyCsTQ+u/kz2Hu/GzRpEUc+yZFgsWlcls89efCuOqAK2/rovgV5n
RuGnMi4BW3b9GAZAeui+wg0nKLDGIGqAekYscdcPaf/sc/GgqvVLgTGNGvc9n5r1
lDkxitY9Mv+xwJWCv3ohRA3TcPgYS4x73Zb7gTDmPYT9/NBxsjU7Ok7ly+Kan8/X
WAPKlX8cxiBx+4xfbSHCFUsy8wWP01SzYzoCeVlbiTDNcFUGBnYHiPO4RJ56h5Hu
lB08tKl42cWzrmGTV0hCxtZlcwXzx+IjsXsva0bnoA8I3Szs8IOXOXLlRXiGcrUN
5Qejk2YJFa+BLt4S9HKWAUgo4IwJcjU9MZ7BUrfjlRDDBlKTnZs=
=gVYV
-----END PGP SIGNATURE-----
"#;
        let buildinfo = Buildinfo::from_str(data).unwrap();
        let mut artifacts = HashSet::new();
        artifacts.insert("courier-base-dbgsym_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-base_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-faxmail_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-imap-dbgsym_5.0.13+1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-imap_5.0.13+1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-ldap-dbgsym_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-ldap_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-mlm-dbgsym_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-mlm_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-mta-dbgsym_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-mta_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-pcp-dbgsym_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-pcp_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-pop-dbgsym_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-pop_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-webadmin-dbgsym_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("courier-webadmin_1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("sqwebmail-dbgsym_6.0.5+1.0.16-3+b1_amd64.deb".to_string());
        artifacts.insert("sqwebmail_6.0.5+1.0.16-3+b1_amd64.deb".to_string());
        assert_eq!(
            buildinfo,
            Buildinfo {
                source: "courier".to_string(),
                version: "1.0.16-3+b1".to_string(),
                artifacts,
            }
        );
    }
}
