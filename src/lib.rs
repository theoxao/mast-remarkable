#![feature(default_free_fn)]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;


pub mod model;
pub mod weather;
pub mod refresh;
pub mod common;
pub mod wifi;
pub mod control;
pub mod battery;