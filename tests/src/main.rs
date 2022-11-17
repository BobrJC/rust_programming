use core::task;
use std::io;
use std::fs::OpenOptions;
use std::io::{Write, BufReader, BufRead, Error};
use std::fs::File;
use std::fs::rename;
use std::ops::Add;
use chrono::FixedOffset;
use chrono::prelude::Local;
use chrono::prelude::DateTime;
use std::collections::HashMap;

fn main() {
    println!("Please, enter the command. For help use \"help\"");
    let path: &str = "tasks.txt";
    let task_vec = vec!(String::from("2022-09-11"), String::from("2022-09-11"), String::from("22:11:11"));
    println!("{:?}", task_vec);
    let dt = DateTime::parse_from_str((task_vec[1].clone().add(&task_vec[2]).add(&Local::now().offset().to_string())).as_str(), "%Y-%m-%d%H:%M:%S%z").unwrap();
    println!("{}", dt.to_string());

    let mut hash: HashMap<_, _, _> = HashMap::from((1, 2));
}