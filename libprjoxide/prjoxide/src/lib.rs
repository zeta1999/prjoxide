use std::fs::File;
use std::io::*;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;


pub mod bba {
    pub mod bbafile;
    pub mod bbastruct;
    pub mod idstring;
    pub mod idxset;
    pub mod tileloc;
    pub mod tiletype;
}

pub mod bels;
pub mod bitstream;
pub mod chip;
pub mod database;
pub mod database_html;
pub mod docs;
pub mod fasmparse;
pub mod fuzz;
pub mod ipfuzz;
pub mod nodecheck;
pub mod wires;