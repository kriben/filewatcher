extern crate getopts;
extern crate glob;
extern crate notify;
extern crate time;

use getopts::Options;
use glob::Pattern;
use notify::{Config, Error, Event, RecommendedWatcher, Watcher};
use std::env;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("d", "directory", "set directory", "DIRECTORY");
    opts.reqopt("p", "pattern", "set pattern", "PATTERN");
    opts.reqopt("c", "command", "set command", "COMMAND");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{0}\n", f);
            print_usage(&program, opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let path = matches.opt_str("d").unwrap_or(String::from("."));
    let command = matches.opt_str("c").unwrap();
    let pattern = match Pattern::new(&matches.opt_str("p").unwrap()) {
        Ok(res) => res,
        Err(msg) => {
            println!("{}\n", msg);
            print_usage(&program, opts);
            return;
        }
    };

    println!("Watching directory: {0}", path);
    println!("Pattern: {0}", pattern.as_str());
    println!("Command: {0}", command);

    let (tx, rx) = channel();
    let watcher = RecommendedWatcher::new(tx, Config::default());
    match watcher {
        Ok(mut w) => {
            let _ = w.watch(path.as_ref(), notify::RecursiveMode::Recursive);
            watch_files(&rx, pattern, &command);
        }
        Err(error) => eprintln!("Error: watch setup failed. {}", error),
    }
}

fn run_command(path: std::path::PathBuf, command: &str) -> String {
    // Split the program and possible arguments into parts
    let mut args = command.split_whitespace();

    // Create command with first argument as the program to run
    let program = args.nth(0).unwrap();
    let mut cmd = Command::new(program);

    // Rest of items should be arguments
    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd
        .arg(path.to_str().unwrap())
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    String::from_utf8(output.stdout).unwrap()
}

fn watch_files(rx: &Receiver<Result<Event, Error>>, pattern: Pattern, command: &str) {
    loop {
        for res in rx {
            match res {
                Ok(event) => {
                    for path in event.paths {
                        if pattern.matches(path.to_str().unwrap()) {
                            let t = time::now();
                            println!(
                                "\n{0}: {1} matched {2}:",
                                t.asctime(),
                                path.to_str().unwrap(),
                                pattern.as_str()
                            );
                            let res = run_command(path, command);
                            println!("{0}", res)
                        }
                    }
                }

                Err(error) => eprintln!("Error: {error:?}"),
            }
        }
    }
}
