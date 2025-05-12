#![allow(dead_code)]
use console::style;

pub(crate) fn green<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).green())
}

pub(crate) fn blue<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).blue())
}

pub(crate) fn yellow<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).yellow())
}

pub(crate) fn red<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).red())
}

pub(crate) fn cyan<T: AsRef<str>>(s: T) {
    println!("{}", style(s.as_ref()).cyan())
}
