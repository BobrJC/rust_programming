// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::fmt;
/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let tmp = format!("{:?}", level).to_ascii_uppercase();
    format!("[{}]: {}", tmp, message)
}
pub fn info(message: &str) -> String {
    String::from(format!("[INFO]: {}", message))
}
pub fn warn(message: &str) -> String {
    unimplemented!("return a message for warn log level")
}
pub fn error(message: &str) -> String {
    unimplemented!("return a message for error log level")
}

pub fn main() {
    println!("{}", log(LogLevel::Error, "ddd"));
    println!("{}", info("ddd"));

}