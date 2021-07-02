use structopt::StructOpt;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum Categories {
    Science = 28,
    People = 22,
    Comedy = 23,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "argparse example", about = "usage example")]
struct Options {
    /// filename of video to upload
    #[structopt(short, long)]
    file: std::path::PathBuf,
    /// title if none given created from filename
    #[structopt(short, long)]
    title: Option<String>,
    /// Video Category
    #[structopt(long, default_value="science", possible_values = &Categories::VARIANTS)]
    category: Categories,
    /// show more output
    #[structopt(short, long)]
    verbose: bool,
    /// provides names to greet
    #[structopt(short = "n", long = "name")]
    names: Vec<String>,
}

fn main() {
    let options = Options::from_args();
    println!("{:?}", options);
}
