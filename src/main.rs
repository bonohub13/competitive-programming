use chrono::prelude::Local;
use competetive_rs as comp_rs;
use logger::Logger;
use std::env;
use std::path::Path;

fn main() -> Result<(), comp_rs::modes::ErrorState> {
    let mut logger = Logger::new();
    let now = Local::now();
    let filename = format!(
        "./log/competitive-rust_{}.log",
        now.format("%Y-%m-%d_%H-%M-%S"),
    );
    let args: Vec<String> = env::args().collect();
    let mode_arg = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("")
    };
    let mode = comp_rs::modes::RunMode::new(Some(mode_arg))?;

    match logger.create_logfile(Path::new(filename.as_str())) {
        Ok(_) => (),
        Err(err) => {
            log::error!("{}", err);

            return Err(comp_rs::modes::ErrorState::FailedToCreateLogFile);
        }
    }
    match logger.init() {
        Ok(_) => (),
        Err(err) => {
            log::error!("{}", err);

            return Err(comp_rs::modes::ErrorState::FailedToInitLogger);
        }
    }

    match 1 {
        0 => {
            // https://atcoder.jp/contests/abc288/tasks/abc288_a
            let mut inputs: Vec<[i64; 2]> = vec![];

            comp_rs::prob_abc288_a::read_user_input(&mut inputs, &mode)?;
            comp_rs::prob_abc288_a::check_ans(&inputs);
        }
        1 => {
            // https://atcoder.jp/contests/abc288/tasks/abc288_b
            let mut inputs: Vec<String> = vec![];

            comp_rs::prob_abc288_b::read_user_input(&mut inputs)?;
            comp_rs::prob_abc288_b::check_ans(&inputs);
        }
        _ => (),
    }

    Ok(())
}
