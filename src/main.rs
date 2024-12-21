/*
 * @Author: FunctionSir
 * @License: AGPLv3
 * @Date: 2024-12-21 21:45:38
 * @LastEditTime: 2024-12-22 00:22:22
 * @LastEditors: FunctionSir
 * @Description: Anyflector ver 0.1.0
 * @FilePath: /anyflector/src/main.rs
 */

use std::{
    env, fs,
    process::{Command, Stdio},
    time::Instant,
    vec,
};

const VER: &str = "0.1.0";
const CODENAME: &str = "Febrie";

struct Args {
    file: String,
    repo: String,
    timeout: i32,
}

struct Entry {
    server: String,
    time: u128,
}

impl Args {
    fn new() -> Args {
        let args: Vec<String> = env::args().collect();
        Args {
            file: args[1].to_owned(),
            repo: args[2].to_owned(),
            timeout: args[3].to_owned().parse().unwrap(),
        }
    }
}

fn hello() {
    eprintln!("Anyflector [Version: {VER} ({CODENAME})]");
    eprintln!("This is a libre software under GNU AGPLv3.");
}

fn speedtest(url: &String, args: &Args) -> Option<u128> {
    let start = Instant::now();
    let result = Command::new("curl")
        .arg(url)
        .arg("--output")
        .arg("/dev/null")
        .arg("--max-time")
        .arg(args.timeout.to_string())
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .status();
    let time_costed = start.elapsed().as_millis();
    if let Ok(status) = result {
        if let Some(code) = status.code() {
            if code != 0 {
                eprintln!("Test for \"{}\" failed: Error: {}", url, code);
                return None;
            }
        } else {
            eprintln!("Test for \"{}\" failed: Error: unknown", url);
            return None;
        }
    }
    eprintln!("Time used downloading \"{}\": {}ms", url, time_costed);
    Some(time_costed)
}

fn main() {
    hello(); // Say hello to users
    let args = Args::new();
    let lines: Vec<String> = fs::read_to_string(&args.file)
        .unwrap()
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    let mut entries = vec![];
    for l in lines {
        if l.trim().starts_with("#") {
            continue;
        }
        if let Some(splited) = l.trim().split_once("=") {
            let server = splited.1.trim().to_string();
            if server.len() <= 0 {
                continue;
            }
            let mut url: String;
            if server.chars().last().unwrap() != '/' {
                url = format!("{server}/{}.db", args.repo);
            } else {
                url = format!("{server}{}.db", args.repo);
            }
            url = url.replace("$repo", &args.repo);
            url = url.replace("$arch", env::consts::ARCH);
            if let Some(time_used) = speedtest(&url, &args) {
                entries.push(Entry {
                    server: server,
                    time: time_used,
                });
            } else {
                continue;
            }
        }
    }
    entries.sort_by(|a, b| a.time.cmp(&b.time));
    let new_lines: Vec<String> = entries
        .iter()
        .map(|x| format!("Server = {}\n", x.server))
        .collect();
    fs::write(&args.file, new_lines.join("")).unwrap();
}
