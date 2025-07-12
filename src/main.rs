mod supdata;
mod video_gpt;

use std::{env, fs};

use clap::{command, Parser};
use dotenv::dotenv;

use crate::video_gpt::{Level, VGConfig, VideoGpt};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    video_url: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long, default_value = "low")]
    level: Level,
}

const SUPDATA_ENV: &str = "SUPADATA_KEY";
const ANTHROPIC_ENV: &str = "ANTHROPIC_KEY";

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let video_url = &cli.video_url;
    let level = cli.level;

    let supdata_key =
        env::var(SUPDATA_ENV).unwrap_or_else(|_| panic!("{} must be set", SUPDATA_ENV));
    let anthropic_key =
        env::var(ANTHROPIC_ENV).unwrap_or_else(|_| panic!("{} must be set", ANTHROPIC_ENV));

    let video_gpt = VideoGpt::new(VGConfig {
        supdata_key,
        anthropic_key,
        level,
    });

    let gpt = video_gpt.get_gpt(video_url).await.unwrap();

    if let Some(output) = cli.output {
        fs::write(output, gpt).unwrap();
    } else {
        println!("{}", gpt);
    }

    println!("Completed.");
}
