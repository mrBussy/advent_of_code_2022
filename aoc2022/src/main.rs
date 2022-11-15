use clap::Parser;
use std::result::Result;
use log::debug;
use aoc2022::exercises::day1::Part1;
use aoc2022::exercises::day1::Part2;
use aoc2022::exercises::Exercise;

const N_DAYS: usize = 1;
const N_PARTS: usize = 1;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run all days or just 1
    #[arg(short, long, default_value_t = false)]
    run_all: bool,

    /// The day to run
    #[arg(short, long, default_value_t = 1)]
    day: u8,

    /// Part of the day to run
    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    env_logger::init();

    if !args.run_all {
        debug!("Do not run all; Start searching for a day");

        if !(0..(N_DAYS+1) as u8).contains(&args.day) {
            return Err(format!("The day must be 1..={}: {}", N_DAYS, args.day));
        }    

        if !(0..(N_PARTS+1) as u8).contains(&args.part) {
            return Err(format!("The day must be 1..={}: {}", N_PARTS, args.part));
        }
    } else {
        debug!("run all days");
        let days: Vec<Box<dyn Exercise>> = vec![Box::new(Part1::new()), Box::new(Part2::new())];

        days.into_iter().for_each( |part| {
            debug!("Part {:?}: {}", part.get_name().unwrap(), part.execute().unwrap());
            }
        );
    }

    Ok(())
} 