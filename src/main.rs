use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use logs_analizer::*;

const LOGS_DIR: &'static str = "./devops/logs";

fn main() {
    let files = get_path_logs_file(LOGS_DIR);
    let mut logs: HashSet<Log> = HashSet::new();
    let mut buffer = String::new();
    let mut reader;
    let mut f;

    for file in files {
        f = File::open(file).unwrap();
        reader = BufReader::new(f);
        let mut log;
        loop {
            buffer.clear();
            reader.read_line(&mut buffer).unwrap();
            if buffer.trim() == "".to_string() {
                break;
            }
            log = Log::new(buffer.clone());
            if log.log_type == LogType::Error {
                println!("=> {:?} \n", &log);
                if logs.insert(log) {
                    println!("Se añadio un elemento");
                } else {
                    println!("No se añadio un elemento");
                }
            }
        }
    }

    let mut matches = HashMap::<&String, u64>::new();

    for log in &logs {
        println!("=> {:?} \n", &log);
        let reps = matches.entry(&log.source_file_name).or_insert(0);
        *reps += 1;
    }

    println!("Logs: {:?}", matches);

    let mut n_groups = HashMap::<&u64, Vec<&String>>::new();
    for (key, val) in matches.iter() {
        let cursor = n_groups.entry(val).or_insert(Vec::new());
        cursor.push(key);
    }
    let mut keys = Vec::new();
    for key in n_groups.keys() {
        keys.push(key);
    }
    keys.sort();
    keys.reverse();

    for key in keys {
        let mut m = n_groups.get(key).unwrap().clone();
        m.sort();
        for n in m {
            println!("{} {}", n, key);
        }
    }

    // let mut matches_vec = Vec::new();
    // for (key, val) in matches.iter() {
    //     matches_vec.push((val, key));
    // }
    // matches_vec.sort_by_key(|k| k.0);
    // matches_vec.reverse();
    // println!("{:?}", matches_vec);
}
