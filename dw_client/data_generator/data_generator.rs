use std::{thread::sleep, time::Duration};

use clap::Parser;
use fake::{Fake, Faker};
use metrics_types::{CounterUnit, FlowUnit, TimerUnit};

#[derive(Parser)]
struct GeneratorArgs {
    rate: usize,

    #[clap(short = 'p', long = "prefix")]
    prefix: Option<String>,
}

fn main() {
    let args = GeneratorArgs::parse();
    let sleep_cnt = if args.rate > 0 {
        1_000_000 / args.rate
    } else {
        1_000_000
    };
    let prefix = args.prefix.unwrap_or(String::from(""));
    loop {
        let f: CounterUnit = Faker.fake();
        // println!("{:?}", f);
        let r = f.revert_to_log();
        println!("{}{}", &prefix, r.to_string());

        let f: TimerUnit = Faker.fake();
        // println!("{:?}", f);
        let r = f.revert_to_log();
        println!("{}{}", &prefix, r.to_string());

        let f: FlowUnit = Faker.fake();
        // println!("{:?}", f);
        let r = f.revert_to_log();
        println!("{}{}", &prefix, r.to_string());

        sleep(Duration::from_micros(sleep_cnt as u64))
    }
}
