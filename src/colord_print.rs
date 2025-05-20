#![allow(dead_code)]
use console::style;

pub fn green<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).green())
}

pub fn blue<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).blue())
}

pub fn yellow<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).yellow())
}

pub fn red<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).red())
}

pub fn cyan<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).cyan())
}
