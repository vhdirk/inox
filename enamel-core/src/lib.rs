#![feature(custom_derive)]
#![feature(custom_attribute)]

use std::rc::Rc;
use std::cell::RefCell;

#[macro_use]
extern crate log;

#[macro_use]
extern crate rental;

extern crate regex;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_ini;
extern crate toml;

extern crate shellexpand;

extern crate notmuch;

pub mod settings;
pub mod database;
