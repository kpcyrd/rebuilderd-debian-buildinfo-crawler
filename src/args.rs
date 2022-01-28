#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(long = "db")]
    pub database: String,
    #[clap(long)]
    pub packages_db: String,
    #[clap(short = 'v', parse(from_occurrences))]
    pub verbose: u8,
    #[clap(long)]
    pub source: String,
    #[clap(long)]
    pub distro: String,
    #[clap(long)]
    pub suite: String,
    #[clap(long)]
    pub skip_crawl: bool,
    #[clap(long = "release")]
    pub releases: Vec<String>,
    #[clap(long = "arch")]
    pub architectures: Vec<String>,
}
