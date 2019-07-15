//! ITM protocol parser.

#![feature(exhaustive_patterns)]
#![feature(generator_trait)]
#![feature(generators)]
#![feature(never_type)]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

pub mod cli;
pub mod itm;
pub mod output;

/// Stimulus ports count.
pub const PORTS_COUNT: usize = 32;

use cli::Cli;
use failure::Error;
use output::Output;
use std::{
    fs::File,
    io::{stdin, Read},
};

impl Cli {
    /// Runs the program.
    pub fn run(&self) -> Result<(), Error> {
        let outputs = Output::open_all(&self.outputs)?;
        let mut parser = itm::Parser::new(&outputs)?;
        if let Some(path) = &self.input {
            for byte in File::open(path)?.bytes() {
                parser.pump(byte?)?;
            }
        } else {
            for byte in stdin().lock().bytes() {
                parser.pump(byte?)?;
            }
        }
        Ok(())
    }
}