#![allow(dead_code)]
use console::style;

pub(crate) fn green(s: &str) {
    println!("{}", style(s).green())
}

pub(crate) fn blue(s: &str) {
    println!("{}", style(s).blue())
}

pub(crate) fn yellow(s: &str) {
    println!("{}", style(s).yellow())
}

pub(crate) fn red(s: &str) {
    println!("{}", style(s).red())
}

pub(crate) fn cyan(s: &str) {
    println!("{}", style(s).cyan())
}
