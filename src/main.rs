extern crate notify;
extern crate glob;
extern crate time;

use notify::{RecommendedWatcher, Error, Watcher, Event};
use std::process::Command;
use glob::Pattern;
use std::sync::mpsc::{channel, Receiver};


fn main() {
    let paths = vec![".", "/tmp/"];
    let command = "wc -l";
    let pattern = Pattern::new("*.txt").unwrap();

    let (tx, rx) = channel();
    let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

    match w {
        Ok(mut watcher) => {
            for p in paths {
                watcher.watch(p).unwrap()
            }
            watch_files(&rx, pattern, command);
        },
        Err(_) => println!("Error: watch setup failed.")
    }
}


fn run_command(path: std::path::PathBuf, command: &str) -> String {

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .arg(path.to_str().unwrap())
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    String::from_utf8(output.stdout).unwrap()
}


fn watch_files(rx: &Receiver<Event>, pattern: Pattern, command: &str) {
    loop {
        match rx.recv() {
            Ok(notify::Event{ path: Some(path), op: Ok(_) }) => {
                if pattern.matches(path.to_str().unwrap()) {
                    let t = time::now();
                    println!("===========================================");
                    println!("{0}: {1} matched {2}:", t.asctime(), path.to_str().unwrap(), pattern.as_str());
                    let res = run_command(path, command);
                    println!("{0}", res)
                }
            },
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }
}
