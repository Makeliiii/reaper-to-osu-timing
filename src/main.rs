use std::fs;
use clap::Parser;

#[derive(Parser)]
#[command(author="Mäkeli", version, about="Convert REAPER timing to osu!", long_about = None)]
struct Args {
    /// Reaper read path (.RPP file)
    read_path: String,

    #[arg(short, long)]
    /// osu! write path (.osu file)
    write_path: Option<String>,

    #[arg(short, long, value_parser = clap::value_parser!(i64).range(0..100))]
    /// Hitobject volume percentage
    volume: Option<i64>,
}

fn run(args: Args) {
    let volume = args.volume.unwrap_or(100);
    let contents: String = fs::read_to_string(args.read_path).expect("Should have been able to read file");
    reaper_to_osu_timing::run(&contents, volume, args.write_path);
}

fn main() {
    let args = Args::parse();
    run(args);
}
