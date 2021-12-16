use std::env::args;
use std::io::stdin;
use std::io::Read;

mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_1a;
mod day_1b;
mod day_2a;
mod day_2b;
mod day_3a;
mod day_3b;
mod day_4a;
mod day_4b;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

const CMDS: &[(&str, fn(&mut dyn Read))] = &[
    ("day-1a", day_1a::run),
    ("day-1b", day_1b::run),
    ("day-2a", day_2a::run),
    ("day-2b", day_2b::run),
    ("day-3a", day_3a::run),
    ("day-3b", day_3b::run),
    ("day-4a", day_4a::run),
    ("day-4b", day_4b::run),
    ("day-5a", day_5::run_part1),
    ("day-5b", day_5::run_part2),
    ("day-6a", day_6::run_part1),
    ("day-6b", day_6::run_part2),
    ("day-7a", day_7::run_part1),
    ("day-7b", day_7::run_part2),
    ("day-8a", day_8::run_part1),
    ("day-8b", day_8::run_part2),
    ("day-9a", day_9::run_part1),
    ("day-9b", day_9::run_part2),
    ("day-10a", day_10::run_part1),
    ("day-10b", day_10::run_part2),
    ("day-11a", day_11::run_part1),
    ("day-11b", day_11::run_part2),
    ("day-12a", day_12::run_part1),
    ("day-12b", day_12::run_part2),
    ("day-12-dot", day_12::run_to_dot),
    ("day-13a", day_13::run_part1),
    ("day-13b", day_13::run_part2),
    ("day-14a", day_14::run_part1),
    ("day-14b", day_14::run_part2),
    ("day-15a", day_15::run_part1),
    ("day-15b", day_15::run_part2),
];

fn find_cmd(name: &str) -> Option<fn(&mut dyn Read)> {
    for (fun_name, fun) in CMDS.iter() {
        if name == *fun_name {
            return Some(*fun);
        }
    }
    return None;
}

fn help(name: &str) {
    println!("usage: {} <subcommand>", name);
    println!("subcommands:");
    for (name, _) in CMDS.iter() {
        println!("    {}", name);
    }
}

fn main() {
    let (subcmd_name, prog_name) = {
        let mut args = args();
        let prog_name = args.next().unwrap();

        let subcmd = match args.next() {
            None => {
                help(&prog_name);
                return;
            }
            Some(cmd) => cmd,
        };
        (subcmd, prog_name)
    };

    let run_fn = match find_cmd(&subcmd_name) {
        Some(f) => f,
        None => {
            println!("Unexpected subcommand name \"{}\"", subcmd_name);
            help(&prog_name);
            return;
        }
    };

    run_fn(&mut stdin())
}
