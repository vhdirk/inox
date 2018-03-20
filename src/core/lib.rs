#![feature(custom_derive)]
#![feature(custom_attribute)]

use std::rc::Rc;
use std::cell::RefCell;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate log;

extern crate regex;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_ini;

extern crate toml;

extern crate shellexpand;

pub mod settings;
