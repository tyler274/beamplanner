#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(portable_simd)]
// mod solution;
pub mod solution_e;
mod test;
mod test_util;
mod util;

use std::{
    collections::{BTreeMap, HashMap},
    env,
    io::Write,
    process::exit,
};

use test::TIMEOUT;
use test_util::{check, BOLD, GRAY, GREEN, RED, RESET, YELLOW};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print!("USAGE: {} OUT_PATH TEST_CASE\n", args[0]);
        exit(1)
    }

    let out_path = &args[1];
    let test_case = &args[2];

    let scenario = test::Scenario::new(test_case).unwrap();

    print!(
        "{GRAY}Scenario: {RESET}{}% coverage ({} users, {} sats){RESET}\n",
        100.0 * scenario.min_coverage,
        scenario.users.len(),
        scenario.sats.len(),
    );

    let start = std::time::Instant::now();
    let solution = solution_e::solve(&scenario.users, &scenario.sats);

    let solution = BTreeMap::from_iter(solution.iter().map(|(k, v)| (*k, *v)));
    let duration = start.elapsed();
    let covered = 1.0 * solution.len() as f32 / scenario.users.len() as f32;

    print!(
        "{GRAY}Solution: {RESET}{BOLD}{}{}% coverage ({} users) in {}{BOLD}{}s{RESET}\n",
        if covered >= scenario.min_coverage {
            GREEN
        } else {
            RED
        },
        100.0 * solution.len() as f32 / scenario.users.len() as f32,
        solution.len(),
        if duration > TIMEOUT {
            RED
        } else {
            if duration > TIMEOUT / 2 {
                YELLOW
            } else {
                GREEN
            }
        },
        duration.as_secs(),
    );

    let mut file = std::fs::File::create(out_path).unwrap();
    file.write(
        format!(
            "{} {} {}s\n",
            test_case,
            100.0 * covered,
            duration.as_secs()
        )
        .as_bytes(),
    )
    .unwrap();

    check(duration < TIMEOUT, "Took too long to produce a solution\n");
    scenario.check(&solution);

    if covered >= scenario.min_coverage {
        exit(0);
    } else {
        exit(1);
    }
}
