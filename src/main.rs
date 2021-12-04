use std::env::args;
use std::io::stdin;
use std::io::Read;

mod day_1a;
mod day_1b;
mod day_2a;
mod day_2b;
mod day_3a;
mod day_3b;
mod day_4a;
mod day_4b;

const CMDS: &[(&str, fn(&mut dyn Read))] = &[
    ("day-1a", day_1a::run),
    ("day-1b", day_1b::run),
    ("day-2a", day_2a::run),
    ("day-2b", day_2b::run),
    ("day-3a", day_3a::run),
    ("day-3b", day_3b::run),
    ("day-4a", day_4a::run),
    ("day-4b", day_4b::run),
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
