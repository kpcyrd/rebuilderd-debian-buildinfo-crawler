use crate::errors::*;
use crate::utils;
use kuchiki::traits::*;
use reqwest::Client;

pub async fn fetch_buildinfo_hrefs(client: &Client, url: &str) -> Result<Vec<String>> {
    let html = utils::fetch_http(client, url).await?;
    parse_buildinfo_hrefs(&html)
}

pub fn parse_buildinfo_hrefs(html: &[u8]) -> Result<Vec<String>> {
    let html = String::from_utf8_lossy(html);
    let document = kuchiki::parse_html().one(html.as_ref());

    let mut out = Vec::new();
    for css_match in document.select("a").unwrap() {
        if let Some(href) = css_match.attributes.borrow().get("href") {
            if href.ends_with(".buildinfo") {
                out.push(href.to_string());
            }
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_directory_list() {
        let html = br#"
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 3.2 Final//EN">
<html>
 <head>
  <title>Index of /buildinfo-pool/r/rust-sniffglue</title>
 </head>
 <body>
<h1>Index of /buildinfo-pool/r/rust-sniffglue</h1>
  <table>
   <tr><th valign="top"><img src="/icons/blank.gif" alt="[ICO]"></th><th><a href="?C=N;O=D">Name</a></th><th><a href="?C=M;O=A">Last modified</a></th><th><a href="?C=S;O=A">Size</a></th><th><a href="?C=D;O=A">Description</a></th></tr>
   <tr><th colspan="5"><hr></th></tr>
<tr><td valign="top"><img src="/icons/back.gif" alt="[PARENTDIR]"></td><td><a href="/buildinfo-pool/r/">Parent Directory</a></td><td>&nbsp;</td><td align="right">  - </td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-1_arm64.buildinfo">rust-sniffglue_0.8.2-1_arm64.buildinfo</a></td><td align="right">2018-11-23 14:26  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-1_armel.buildinfo">rust-sniffglue_0.8.2-1_armel.buildinfo</a></td><td align="right">2018-11-24 01:43  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-1_i386.buildinfo">rust-sniffglue_0.8.2-1_i386.buildinfo</a></td><td align="right">2018-11-23 19:33  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-1_ppc64el.buildinfo">rust-sniffglue_0.8.2-1_ppc64el.buildinfo</a></td><td align="right">2018-11-23 14:11  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-1_s390x.buildinfo">rust-sniffglue_0.8.2-1_s390x.buildinfo</a></td><td align="right">2018-11-23 17:24  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-2_amd64-source.buildinfo">rust-sniffglue_0.8.2-2_amd64-source.buildinfo</a></td><td align="right">2018-12-15 09:52  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-2_amd64.buildinfo">rust-sniffglue_0.8.2-2_amd64.buildinfo</a></td><td align="right">2018-12-15 10:42  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-2_arm64.buildinfo">rust-sniffglue_0.8.2-2_arm64.buildinfo</a></td><td align="right">2018-12-15 10:42  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-2_armel.buildinfo">rust-sniffglue_0.8.2-2_armel.buildinfo</a></td><td align="right">2018-12-15 11:43  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-2_armhf.buildinfo">rust-sniffglue_0.8.2-2_armhf.buildinfo</a></td><td align="right">2018-12-15 10:57  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-2_i386.buildinfo">rust-sniffglue_0.8.2-2_i386.buildinfo</a></td><td align="right">2018-12-15 10:42  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-2_mips.buildinfo">rust-sniffglue_0.8.2-2_mips.buildinfo</a></td><td align="right">2018-12-15 11:13  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-2_ppc64el.buildinfo">rust-sniffglue_0.8.2-2_ppc64el.buildinfo</a></td><td align="right">2018-12-15 10:42  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-2_s390x.buildinfo">rust-sniffglue_0.8.2-2_s390x.buildinfo</a></td><td align="right">2018-12-15 10:32  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_amd64.buildinfo">rust-sniffglue_0.8.2-3_amd64.buildinfo</a></td><td align="right">2018-12-21 23:28  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_arm64.buildinfo">rust-sniffglue_0.8.2-3_arm64.buildinfo</a></td><td align="right">2018-12-22 00:40  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_armel.buildinfo">rust-sniffglue_0.8.2-3_armel.buildinfo</a></td><td align="right">2018-12-22 00:55  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_armhf.buildinfo">rust-sniffglue_0.8.2-3_armhf.buildinfo</a></td><td align="right">2018-12-22 01:10  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_i386.buildinfo">rust-sniffglue_0.8.2-3_i386.buildinfo</a></td><td align="right">2018-12-22 00:25  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_mips.buildinfo">rust-sniffglue_0.8.2-3_mips.buildinfo</a></td><td align="right">2018-12-22 01:10  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_mips64el.buildinfo">rust-sniffglue_0.8.2-3_mips64el.buildinfo</a></td><td align="right">2018-12-29 01:44  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_mipsel.buildinfo">rust-sniffglue_0.8.2-3_mipsel.buildinfo</a></td><td align="right">2018-12-27 07:43  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_ppc64el.buildinfo">rust-sniffglue_0.8.2-3_ppc64el.buildinfo</a></td><td align="right">2018-12-22 00:30  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_s390x.buildinfo">rust-sniffglue_0.8.2-3_s390x.buildinfo</a></td><td align="right">2018-12-22 00:30  </td><td align="right"> 10K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-3_source.buildinfo">rust-sniffglue_0.8.2-3_source.buildinfo</a></td><td align="right">2018-12-19 01:41  </td><td align="right">5.8K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_amd64-source.buildinfo">rust-sniffglue_0.8.2-4_amd64-source.buildinfo</a></td><td align="right">2019-02-08 08:54  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_amd64.buildinfo">rust-sniffglue_0.8.2-4_amd64.buildinfo</a></td><td align="right">2019-02-08 12:43  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_arm64.buildinfo">rust-sniffglue_0.8.2-4_arm64.buildinfo</a></td><td align="right">2019-02-08 10:00  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_armel.buildinfo">rust-sniffglue_0.8.2-4_armel.buildinfo</a></td><td align="right">2019-02-08 11:11  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_armhf.buildinfo">rust-sniffglue_0.8.2-4_armhf.buildinfo</a></td><td align="right">2019-02-08 12:11  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_i386.buildinfo">rust-sniffglue_0.8.2-4_i386.buildinfo</a></td><td align="right">2019-02-08 11:41  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_mips.buildinfo">rust-sniffglue_0.8.2-4_mips.buildinfo</a></td><td align="right">2019-02-08 10:55  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_mips64el.buildinfo">rust-sniffglue_0.8.2-4_mips64el.buildinfo</a></td><td align="right">2019-02-08 10:40  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_mipsel.buildinfo">rust-sniffglue_0.8.2-4_mipsel.buildinfo</a></td><td align="right">2019-02-08 11:11  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_ppc64el.buildinfo">rust-sniffglue_0.8.2-4_ppc64el.buildinfo</a></td><td align="right">2019-02-08 10:00  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.8.2-4_s390x.buildinfo">rust-sniffglue_0.8.2-4_s390x.buildinfo</a></td><td align="right">2019-02-08 09:55  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.9.0-1_amd64.buildinfo">rust-sniffglue_0.9.0-1_amd64.buildinfo</a></td><td align="right">2019-09-11 23:43  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.9.0-1_arm64.buildinfo">rust-sniffglue_0.9.0-1_arm64.buildinfo</a></td><td align="right">2019-09-11 22:42  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.9.0-1_armel.buildinfo">rust-sniffglue_0.9.0-1_armel.buildinfo</a></td><td align="right">2019-09-12 02:57  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.9.0-1_armhf.buildinfo">rust-sniffglue_0.9.0-1_armhf.buildinfo</a></td><td align="right">2019-09-12 00:44  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.9.0-1_i386.buildinfo">rust-sniffglue_0.9.0-1_i386.buildinfo</a></td><td align="right">2019-09-11 23:28  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.9.0-1_mips64el.buildinfo">rust-sniffglue_0.9.0-1_mips64el.buildinfo</a></td><td align="right">2019-09-29 14:58  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.9.0-1_mipsel.buildinfo">rust-sniffglue_0.9.0-1_mipsel.buildinfo</a></td><td align="right">2019-09-29 13:42  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.9.0-1_ppc64el.buildinfo">rust-sniffglue_0.9.0-1_ppc64el.buildinfo</a></td><td align="right">2019-09-11 22:42  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.9.0-1_source.buildinfo">rust-sniffglue_0.9.0-1_source.buildinfo</a></td><td align="right">2019-09-11 22:02  </td><td align="right">6.5K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.10.1-1_amd64.buildinfo">rust-sniffglue_0.10.1-1_amd64.buildinfo</a></td><td align="right">2020-01-01 10:56  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.10.1-1_armel.buildinfo">rust-sniffglue_0.10.1-1_armel.buildinfo</a></td><td align="right">2020-01-01 11:12  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.10.1-1_armhf.buildinfo">rust-sniffglue_0.10.1-1_armhf.buildinfo</a></td><td align="right">2020-01-01 10:41  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.10.1-1_i386.buildinfo">rust-sniffglue_0.10.1-1_i386.buildinfo</a></td><td align="right">2020-01-01 10:56  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.10.1-1_ppc64el.buildinfo">rust-sniffglue_0.10.1-1_ppc64el.buildinfo</a></td><td align="right">2020-01-01 10:41  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.10.1-1_source.buildinfo">rust-sniffglue_0.10.1-1_source.buildinfo</a></td><td align="right">2020-01-01 07:35  </td><td align="right">6.5K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-1_amd64.buildinfo">rust-sniffglue_0.11.1-1_amd64.buildinfo</a></td><td align="right">2020-10-05 18:25  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-1_arm64.buildinfo">rust-sniffglue_0.11.1-1_arm64.buildinfo</a></td><td align="right">2020-10-05 18:25  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-1_armel.buildinfo">rust-sniffglue_0.11.1-1_armel.buildinfo</a></td><td align="right">2020-10-05 18:55  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-1_armhf.buildinfo">rust-sniffglue_0.11.1-1_armhf.buildinfo</a></td><td align="right">2020-10-05 18:55  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-1_i386.buildinfo">rust-sniffglue_0.11.1-1_i386.buildinfo</a></td><td align="right">2020-10-05 18:20  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-1_s390x.buildinfo">rust-sniffglue_0.11.1-1_s390x.buildinfo</a></td><td align="right">2020-10-05 18:20  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-1_source.buildinfo">rust-sniffglue_0.11.1-1_source.buildinfo</a></td><td align="right">2020-10-05 17:49  </td><td align="right">6.6K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_amd64.buildinfo">rust-sniffglue_0.11.1-2_amd64.buildinfo</a></td><td align="right">2020-10-06 13:12  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_arm64.buildinfo">rust-sniffglue_0.11.1-2_arm64.buildinfo</a></td><td align="right">2020-10-06 13:12  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_armel.buildinfo">rust-sniffglue_0.11.1-2_armel.buildinfo</a></td><td align="right">2020-10-06 13:43  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_armhf.buildinfo">rust-sniffglue_0.11.1-2_armhf.buildinfo</a></td><td align="right">2020-10-06 13:12  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_i386.buildinfo">rust-sniffglue_0.11.1-2_i386.buildinfo</a></td><td align="right">2020-10-06 13:12  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_mips64el.buildinfo">rust-sniffglue_0.11.1-2_mips64el.buildinfo</a></td><td align="right">2020-10-06 13:58  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_mipsel.buildinfo">rust-sniffglue_0.11.1-2_mipsel.buildinfo</a></td><td align="right">2020-10-06 13:58  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_ppc64el.buildinfo">rust-sniffglue_0.11.1-2_ppc64el.buildinfo</a></td><td align="right">2020-10-06 13:12  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_s390x.buildinfo">rust-sniffglue_0.11.1-2_s390x.buildinfo</a></td><td align="right">2020-10-06 13:07  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-2_source.buildinfo">rust-sniffglue_0.11.1-2_source.buildinfo</a></td><td align="right">2020-10-06 12:47  </td><td align="right">6.6K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_amd64.buildinfo">rust-sniffglue_0.11.1-4_amd64.buildinfo</a></td><td align="right">2020-11-05 16:14  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_arm64.buildinfo">rust-sniffglue_0.11.1-4_arm64.buildinfo</a></td><td align="right">2020-11-05 16:15  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_armel.buildinfo">rust-sniffglue_0.11.1-4_armel.buildinfo</a></td><td align="right">2020-11-05 16:30  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_armhf.buildinfo">rust-sniffglue_0.11.1-4_armhf.buildinfo</a></td><td align="right">2020-11-05 16:14  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_i386.buildinfo">rust-sniffglue_0.11.1-4_i386.buildinfo</a></td><td align="right">2020-11-05 16:15  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_mips64el.buildinfo">rust-sniffglue_0.11.1-4_mips64el.buildinfo</a></td><td align="right">2020-11-05 16:45  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_mipsel.buildinfo">rust-sniffglue_0.11.1-4_mipsel.buildinfo</a></td><td align="right">2020-11-05 16:55  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_ppc64el.buildinfo">rust-sniffglue_0.11.1-4_ppc64el.buildinfo</a></td><td align="right">2020-11-05 16:14  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_s390x.buildinfo">rust-sniffglue_0.11.1-4_s390x.buildinfo</a></td><td align="right">2020-11-05 16:14  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-4_source.buildinfo">rust-sniffglue_0.11.1-4_source.buildinfo</a></td><td align="right">2020-11-05 15:34  </td><td align="right">6.8K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5+b1_amd64.buildinfo">rust-sniffglue_0.11.1-5+b1_amd64.buildinfo</a></td><td align="right">2021-01-20 04:12  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5+b1_arm64.buildinfo">rust-sniffglue_0.11.1-5+b1_arm64.buildinfo</a></td><td align="right">2021-01-19 17:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5+b1_armel.buildinfo">rust-sniffglue_0.11.1-5+b1_armel.buildinfo</a></td><td align="right">2021-01-19 17:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5+b1_armhf.buildinfo">rust-sniffglue_0.11.1-5+b1_armhf.buildinfo</a></td><td align="right">2021-01-19 19:12  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5+b1_i386.buildinfo">rust-sniffglue_0.11.1-5+b1_i386.buildinfo</a></td><td align="right">2021-01-20 02:14  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5+b1_mips64el.buildinfo">rust-sniffglue_0.11.1-5+b1_mips64el.buildinfo</a></td><td align="right">2021-01-23 08:14  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5+b1_mipsel.buildinfo">rust-sniffglue_0.11.1-5+b1_mipsel.buildinfo</a></td><td align="right">2021-01-24 17:40  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5+b1_ppc64el.buildinfo">rust-sniffglue_0.11.1-5+b1_ppc64el.buildinfo</a></td><td align="right">2021-01-21 10:59  </td><td align="right"> 14K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5+b1_s390x.buildinfo">rust-sniffglue_0.11.1-5+b1_s390x.buildinfo</a></td><td align="right">2021-01-21 14:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_amd64.buildinfo">rust-sniffglue_0.11.1-5_amd64.buildinfo</a></td><td align="right">2020-11-27 00:29  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_arm64.buildinfo">rust-sniffglue_0.11.1-5_arm64.buildinfo</a></td><td align="right">2020-12-04 22:59  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_armel.buildinfo">rust-sniffglue_0.11.1-5_armel.buildinfo</a></td><td align="right">2020-12-04 23:14  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_armhf.buildinfo">rust-sniffglue_0.11.1-5_armhf.buildinfo</a></td><td align="right">2020-12-04 22:59  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_i386.buildinfo">rust-sniffglue_0.11.1-5_i386.buildinfo</a></td><td align="right">2020-11-27 01:42  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_mips64el.buildinfo">rust-sniffglue_0.11.1-5_mips64el.buildinfo</a></td><td align="right">2020-12-05 19:10  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_mipsel.buildinfo">rust-sniffglue_0.11.1-5_mipsel.buildinfo</a></td><td align="right">2020-12-05 20:56  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_ppc64el.buildinfo">rust-sniffglue_0.11.1-5_ppc64el.buildinfo</a></td><td align="right">2020-12-04 22:59  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_s390x.buildinfo">rust-sniffglue_0.11.1-5_s390x.buildinfo</a></td><td align="right">2020-12-05 18:55  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-5_source.buildinfo">rust-sniffglue_0.11.1-5_source.buildinfo</a></td><td align="right">2020-11-25 22:43  </td><td align="right">8.5K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6+b1_amd64.buildinfo">rust-sniffglue_0.11.1-6+b1_amd64.buildinfo</a></td><td align="right">2021-04-21 20:10  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6+b1_arm64.buildinfo">rust-sniffglue_0.11.1-6+b1_arm64.buildinfo</a></td><td align="right">2021-04-21 19:44  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6+b1_armel.buildinfo">rust-sniffglue_0.11.1-6+b1_armel.buildinfo</a></td><td align="right">2021-04-21 19:59  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6+b1_armhf.buildinfo">rust-sniffglue_0.11.1-6+b1_armhf.buildinfo</a></td><td align="right">2021-04-21 19:44  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6+b1_i386.buildinfo">rust-sniffglue_0.11.1-6+b1_i386.buildinfo</a></td><td align="right">2021-04-21 20:25  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6+b1_mips64el.buildinfo">rust-sniffglue_0.11.1-6+b1_mips64el.buildinfo</a></td><td align="right">2021-04-21 21:40  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6+b1_mipsel.buildinfo">rust-sniffglue_0.11.1-6+b1_mipsel.buildinfo</a></td><td align="right">2021-04-21 23:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6+b1_ppc64el.buildinfo">rust-sniffglue_0.11.1-6+b1_ppc64el.buildinfo</a></td><td align="right">2021-04-21 19:44  </td><td align="right"> 14K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6+b1_s390x.buildinfo">rust-sniffglue_0.11.1-6+b1_s390x.buildinfo</a></td><td align="right">2021-04-22 04:41  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_amd64.buildinfo">rust-sniffglue_0.11.1-6_amd64.buildinfo</a></td><td align="right">2021-03-26 21:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_arm64.buildinfo">rust-sniffglue_0.11.1-6_arm64.buildinfo</a></td><td align="right">2021-03-26 21:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_armel.buildinfo">rust-sniffglue_0.11.1-6_armel.buildinfo</a></td><td align="right">2021-03-26 21:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_armhf.buildinfo">rust-sniffglue_0.11.1-6_armhf.buildinfo</a></td><td align="right">2021-03-26 21:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_i386.buildinfo">rust-sniffglue_0.11.1-6_i386.buildinfo</a></td><td align="right">2021-03-26 21:13  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_mips64el.buildinfo">rust-sniffglue_0.11.1-6_mips64el.buildinfo</a></td><td align="right">2021-03-26 21:42  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_mipsel.buildinfo">rust-sniffglue_0.11.1-6_mipsel.buildinfo</a></td><td align="right">2021-03-26 21:42  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_ppc64el.buildinfo">rust-sniffglue_0.11.1-6_ppc64el.buildinfo</a></td><td align="right">2021-03-26 21:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_s390x.buildinfo">rust-sniffglue_0.11.1-6_s390x.buildinfo</a></td><td align="right">2021-03-26 21:42  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.11.1-6_source.buildinfo">rust-sniffglue_0.11.1-6_source.buildinfo</a></td><td align="right">2021-03-26 20:58  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-1_source.buildinfo">rust-sniffglue_0.12.1-1_source.buildinfo</a></td><td align="right">2021-08-22 14:09  </td><td align="right"> 11K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_amd64.buildinfo">rust-sniffglue_0.12.1-2_amd64.buildinfo</a></td><td align="right">2021-08-23 10:25  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_arm64.buildinfo">rust-sniffglue_0.12.1-2_arm64.buildinfo</a></td><td align="right">2021-08-23 10:25  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_armel.buildinfo">rust-sniffglue_0.12.1-2_armel.buildinfo</a></td><td align="right">2021-08-23 10:25  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_armhf.buildinfo">rust-sniffglue_0.12.1-2_armhf.buildinfo</a></td><td align="right">2021-08-23 10:25  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_i386.buildinfo">rust-sniffglue_0.12.1-2_i386.buildinfo</a></td><td align="right">2021-08-23 10:10  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_mips64el.buildinfo">rust-sniffglue_0.12.1-2_mips64el.buildinfo</a></td><td align="right">2021-08-23 19:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_mipsel.buildinfo">rust-sniffglue_0.12.1-2_mipsel.buildinfo</a></td><td align="right">2021-08-23 10:41  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_ppc64el.buildinfo">rust-sniffglue_0.12.1-2_ppc64el.buildinfo</a></td><td align="right">2021-08-23 10:25  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_s390x.buildinfo">rust-sniffglue_0.12.1-2_s390x.buildinfo</a></td><td align="right">2021-08-23 10:10  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.12.1-2_source.buildinfo">rust-sniffglue_0.12.1-2_source.buildinfo</a></td><td align="right">2021-08-23 09:45  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_amd64.buildinfo">rust-sniffglue_0.14.0-1_amd64.buildinfo</a></td><td align="right">2021-10-28 17:43  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_arm64.buildinfo">rust-sniffglue_0.14.0-1_arm64.buildinfo</a></td><td align="right">2021-10-28 16:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_armel.buildinfo">rust-sniffglue_0.14.0-1_armel.buildinfo</a></td><td align="right">2021-10-28 16:43  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_armhf.buildinfo">rust-sniffglue_0.14.0-1_armhf.buildinfo</a></td><td align="right">2021-10-28 16:43  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_i386.buildinfo">rust-sniffglue_0.14.0-1_i386.buildinfo</a></td><td align="right">2021-10-28 17:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_mips64el.buildinfo">rust-sniffglue_0.14.0-1_mips64el.buildinfo</a></td><td align="right">2021-10-28 17:13  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_mipsel.buildinfo">rust-sniffglue_0.14.0-1_mipsel.buildinfo</a></td><td align="right">2021-10-28 16:58  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_ppc64el.buildinfo">rust-sniffglue_0.14.0-1_ppc64el.buildinfo</a></td><td align="right">2021-10-28 17:13  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_s390x.buildinfo">rust-sniffglue_0.14.0-1_s390x.buildinfo</a></td><td align="right">2021-10-28 17:28  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-1_source.buildinfo">rust-sniffglue_0.14.0-1_source.buildinfo</a></td><td align="right">2021-10-28 16:03  </td><td align="right"> 13K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_amd64.buildinfo">rust-sniffglue_0.14.0-2_amd64.buildinfo</a></td><td align="right">2021-12-06 21:42  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_arm64.buildinfo">rust-sniffglue_0.14.0-2_arm64.buildinfo</a></td><td align="right">2021-12-06 21:57  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_armel.buildinfo">rust-sniffglue_0.14.0-2_armel.buildinfo</a></td><td align="right">2021-12-06 21:57  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_armhf.buildinfo">rust-sniffglue_0.14.0-2_armhf.buildinfo</a></td><td align="right">2021-12-06 21:57  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_i386.buildinfo">rust-sniffglue_0.14.0-2_i386.buildinfo</a></td><td align="right">2021-12-06 21:37  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_mips64el.buildinfo">rust-sniffglue_0.14.0-2_mips64el.buildinfo</a></td><td align="right">2021-12-06 22:12  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_mipsel.buildinfo">rust-sniffglue_0.14.0-2_mipsel.buildinfo</a></td><td align="right">2021-12-06 21:57  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_ppc64el.buildinfo">rust-sniffglue_0.14.0-2_ppc64el.buildinfo</a></td><td align="right">2021-12-06 21:42  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_s390x.buildinfo">rust-sniffglue_0.14.0-2_s390x.buildinfo</a></td><td align="right">2021-12-06 21:37  </td><td align="right"> 12K</td><td>&nbsp;</td></tr>
<tr><td valign="top"><img src="/icons/unknown.gif" alt="[   ]"></td><td><a href="rust-sniffglue_0.14.0-2_source.buildinfo">rust-sniffglue_0.14.0-2_source.buildinfo</a></td><td align="right">2021-12-06 21:06  </td><td align="right">6.9K</td><td>&nbsp;</td></tr>
   <tr><th colspan="5"><hr></th></tr>
</table>
<address>Apache/2.4.52 (Debian) Server at buildinfos.debian.net Port 443</address>
</body></html>
"#;
        let hrefs = parse_buildinfo_hrefs(html).unwrap();
        assert_eq!(
            &hrefs,
            &[
                "rust-sniffglue_0.8.2-1_arm64.buildinfo",
                "rust-sniffglue_0.8.2-1_armel.buildinfo",
                "rust-sniffglue_0.8.2-1_i386.buildinfo",
                "rust-sniffglue_0.8.2-1_ppc64el.buildinfo",
                "rust-sniffglue_0.8.2-1_s390x.buildinfo",
                "rust-sniffglue_0.8.2-2_amd64-source.buildinfo",
                "rust-sniffglue_0.8.2-2_amd64.buildinfo",
                "rust-sniffglue_0.8.2-2_arm64.buildinfo",
                "rust-sniffglue_0.8.2-2_armel.buildinfo",
                "rust-sniffglue_0.8.2-2_armhf.buildinfo",
                "rust-sniffglue_0.8.2-2_i386.buildinfo",
                "rust-sniffglue_0.8.2-2_mips.buildinfo",
                "rust-sniffglue_0.8.2-2_ppc64el.buildinfo",
                "rust-sniffglue_0.8.2-2_s390x.buildinfo",
                "rust-sniffglue_0.8.2-3_amd64.buildinfo",
                "rust-sniffglue_0.8.2-3_arm64.buildinfo",
                "rust-sniffglue_0.8.2-3_armel.buildinfo",
                "rust-sniffglue_0.8.2-3_armhf.buildinfo",
                "rust-sniffglue_0.8.2-3_i386.buildinfo",
                "rust-sniffglue_0.8.2-3_mips.buildinfo",
                "rust-sniffglue_0.8.2-3_mips64el.buildinfo",
                "rust-sniffglue_0.8.2-3_mipsel.buildinfo",
                "rust-sniffglue_0.8.2-3_ppc64el.buildinfo",
                "rust-sniffglue_0.8.2-3_s390x.buildinfo",
                "rust-sniffglue_0.8.2-3_source.buildinfo",
                "rust-sniffglue_0.8.2-4_amd64-source.buildinfo",
                "rust-sniffglue_0.8.2-4_amd64.buildinfo",
                "rust-sniffglue_0.8.2-4_arm64.buildinfo",
                "rust-sniffglue_0.8.2-4_armel.buildinfo",
                "rust-sniffglue_0.8.2-4_armhf.buildinfo",
                "rust-sniffglue_0.8.2-4_i386.buildinfo",
                "rust-sniffglue_0.8.2-4_mips.buildinfo",
                "rust-sniffglue_0.8.2-4_mips64el.buildinfo",
                "rust-sniffglue_0.8.2-4_mipsel.buildinfo",
                "rust-sniffglue_0.8.2-4_ppc64el.buildinfo",
                "rust-sniffglue_0.8.2-4_s390x.buildinfo",
                "rust-sniffglue_0.9.0-1_amd64.buildinfo",
                "rust-sniffglue_0.9.0-1_arm64.buildinfo",
                "rust-sniffglue_0.9.0-1_armel.buildinfo",
                "rust-sniffglue_0.9.0-1_armhf.buildinfo",
                "rust-sniffglue_0.9.0-1_i386.buildinfo",
                "rust-sniffglue_0.9.0-1_mips64el.buildinfo",
                "rust-sniffglue_0.9.0-1_mipsel.buildinfo",
                "rust-sniffglue_0.9.0-1_ppc64el.buildinfo",
                "rust-sniffglue_0.9.0-1_source.buildinfo",
                "rust-sniffglue_0.10.1-1_amd64.buildinfo",
                "rust-sniffglue_0.10.1-1_armel.buildinfo",
                "rust-sniffglue_0.10.1-1_armhf.buildinfo",
                "rust-sniffglue_0.10.1-1_i386.buildinfo",
                "rust-sniffglue_0.10.1-1_ppc64el.buildinfo",
                "rust-sniffglue_0.10.1-1_source.buildinfo",
                "rust-sniffglue_0.11.1-1_amd64.buildinfo",
                "rust-sniffglue_0.11.1-1_arm64.buildinfo",
                "rust-sniffglue_0.11.1-1_armel.buildinfo",
                "rust-sniffglue_0.11.1-1_armhf.buildinfo",
                "rust-sniffglue_0.11.1-1_i386.buildinfo",
                "rust-sniffglue_0.11.1-1_s390x.buildinfo",
                "rust-sniffglue_0.11.1-1_source.buildinfo",
                "rust-sniffglue_0.11.1-2_amd64.buildinfo",
                "rust-sniffglue_0.11.1-2_arm64.buildinfo",
                "rust-sniffglue_0.11.1-2_armel.buildinfo",
                "rust-sniffglue_0.11.1-2_armhf.buildinfo",
                "rust-sniffglue_0.11.1-2_i386.buildinfo",
                "rust-sniffglue_0.11.1-2_mips64el.buildinfo",
                "rust-sniffglue_0.11.1-2_mipsel.buildinfo",
                "rust-sniffglue_0.11.1-2_ppc64el.buildinfo",
                "rust-sniffglue_0.11.1-2_s390x.buildinfo",
                "rust-sniffglue_0.11.1-2_source.buildinfo",
                "rust-sniffglue_0.11.1-4_amd64.buildinfo",
                "rust-sniffglue_0.11.1-4_arm64.buildinfo",
                "rust-sniffglue_0.11.1-4_armel.buildinfo",
                "rust-sniffglue_0.11.1-4_armhf.buildinfo",
                "rust-sniffglue_0.11.1-4_i386.buildinfo",
                "rust-sniffglue_0.11.1-4_mips64el.buildinfo",
                "rust-sniffglue_0.11.1-4_mipsel.buildinfo",
                "rust-sniffglue_0.11.1-4_ppc64el.buildinfo",
                "rust-sniffglue_0.11.1-4_s390x.buildinfo",
                "rust-sniffglue_0.11.1-4_source.buildinfo",
                "rust-sniffglue_0.11.1-5+b1_amd64.buildinfo",
                "rust-sniffglue_0.11.1-5+b1_arm64.buildinfo",
                "rust-sniffglue_0.11.1-5+b1_armel.buildinfo",
                "rust-sniffglue_0.11.1-5+b1_armhf.buildinfo",
                "rust-sniffglue_0.11.1-5+b1_i386.buildinfo",
                "rust-sniffglue_0.11.1-5+b1_mips64el.buildinfo",
                "rust-sniffglue_0.11.1-5+b1_mipsel.buildinfo",
                "rust-sniffglue_0.11.1-5+b1_ppc64el.buildinfo",
                "rust-sniffglue_0.11.1-5+b1_s390x.buildinfo",
                "rust-sniffglue_0.11.1-5_amd64.buildinfo",
                "rust-sniffglue_0.11.1-5_arm64.buildinfo",
                "rust-sniffglue_0.11.1-5_armel.buildinfo",
                "rust-sniffglue_0.11.1-5_armhf.buildinfo",
                "rust-sniffglue_0.11.1-5_i386.buildinfo",
                "rust-sniffglue_0.11.1-5_mips64el.buildinfo",
                "rust-sniffglue_0.11.1-5_mipsel.buildinfo",
                "rust-sniffglue_0.11.1-5_ppc64el.buildinfo",
                "rust-sniffglue_0.11.1-5_s390x.buildinfo",
                "rust-sniffglue_0.11.1-5_source.buildinfo",
                "rust-sniffglue_0.11.1-6+b1_amd64.buildinfo",
                "rust-sniffglue_0.11.1-6+b1_arm64.buildinfo",
                "rust-sniffglue_0.11.1-6+b1_armel.buildinfo",
                "rust-sniffglue_0.11.1-6+b1_armhf.buildinfo",
                "rust-sniffglue_0.11.1-6+b1_i386.buildinfo",
                "rust-sniffglue_0.11.1-6+b1_mips64el.buildinfo",
                "rust-sniffglue_0.11.1-6+b1_mipsel.buildinfo",
                "rust-sniffglue_0.11.1-6+b1_ppc64el.buildinfo",
                "rust-sniffglue_0.11.1-6+b1_s390x.buildinfo",
                "rust-sniffglue_0.11.1-6_amd64.buildinfo",
                "rust-sniffglue_0.11.1-6_arm64.buildinfo",
                "rust-sniffglue_0.11.1-6_armel.buildinfo",
                "rust-sniffglue_0.11.1-6_armhf.buildinfo",
                "rust-sniffglue_0.11.1-6_i386.buildinfo",
                "rust-sniffglue_0.11.1-6_mips64el.buildinfo",
                "rust-sniffglue_0.11.1-6_mipsel.buildinfo",
                "rust-sniffglue_0.11.1-6_ppc64el.buildinfo",
                "rust-sniffglue_0.11.1-6_s390x.buildinfo",
                "rust-sniffglue_0.11.1-6_source.buildinfo",
                "rust-sniffglue_0.12.1-1_source.buildinfo",
                "rust-sniffglue_0.12.1-2_amd64.buildinfo",
                "rust-sniffglue_0.12.1-2_arm64.buildinfo",
                "rust-sniffglue_0.12.1-2_armel.buildinfo",
                "rust-sniffglue_0.12.1-2_armhf.buildinfo",
                "rust-sniffglue_0.12.1-2_i386.buildinfo",
                "rust-sniffglue_0.12.1-2_mips64el.buildinfo",
                "rust-sniffglue_0.12.1-2_mipsel.buildinfo",
                "rust-sniffglue_0.12.1-2_ppc64el.buildinfo",
                "rust-sniffglue_0.12.1-2_s390x.buildinfo",
                "rust-sniffglue_0.12.1-2_source.buildinfo",
                "rust-sniffglue_0.14.0-1_amd64.buildinfo",
                "rust-sniffglue_0.14.0-1_arm64.buildinfo",
                "rust-sniffglue_0.14.0-1_armel.buildinfo",
                "rust-sniffglue_0.14.0-1_armhf.buildinfo",
                "rust-sniffglue_0.14.0-1_i386.buildinfo",
                "rust-sniffglue_0.14.0-1_mips64el.buildinfo",
                "rust-sniffglue_0.14.0-1_mipsel.buildinfo",
                "rust-sniffglue_0.14.0-1_ppc64el.buildinfo",
                "rust-sniffglue_0.14.0-1_s390x.buildinfo",
                "rust-sniffglue_0.14.0-1_source.buildinfo",
                "rust-sniffglue_0.14.0-2_amd64.buildinfo",
                "rust-sniffglue_0.14.0-2_arm64.buildinfo",
                "rust-sniffglue_0.14.0-2_armel.buildinfo",
                "rust-sniffglue_0.14.0-2_armhf.buildinfo",
                "rust-sniffglue_0.14.0-2_i386.buildinfo",
                "rust-sniffglue_0.14.0-2_mips64el.buildinfo",
                "rust-sniffglue_0.14.0-2_mipsel.buildinfo",
                "rust-sniffglue_0.14.0-2_ppc64el.buildinfo",
                "rust-sniffglue_0.14.0-2_s390x.buildinfo",
                "rust-sniffglue_0.14.0-2_source.buildinfo"
            ]
        );
    }
}
