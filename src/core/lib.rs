#![feature(custom_attribute)]

use std::rc::Rc;
use std::cell::RefCell;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate regex;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate toml;

pub mod settings;
