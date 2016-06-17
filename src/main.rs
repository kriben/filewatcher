extern crate notify;
extern crate glob;

use notify::{RecommendedWatcher, Error, Watcher, Event};
use std::process::Command;
use glob::Pattern;
use std::sync::mpsc::{channel, Receiver};


fn main() {
    let paths = vec![".", "/tmp/"];
    let command = "echo hello";
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


fn watch_files(rx: &Receiver<Event>, pattern: Pattern, command: &str) {
    loop {
        match rx.recv() {
            Ok(notify::Event{ path: Some(path), op: Ok(op) }) => {
                if pattern.matches(path.to_str().unwrap()) {
                    println!("{:?} {:?}", path, op);
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .output()
                        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
                    println!("{:?}", String::from_utf8(output.stdout))
                }
                else {
                    println!("No match");
                }
            },
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }
}
