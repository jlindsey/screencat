/* 
 * screencat - use sendcat.com to share screenshots automatically
 * Copyright (C) 2018  Josh Lindsey <joshua.s.lindsey@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate clipboard;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate notify;
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate shellexpand;

pub mod errors;
pub mod fs_watcher;

use log::LevelFilter;
use env_logger::Builder;
use std::env;
use std::io::Write;

pub fn init_logging() {
    let mut builder = Builder::new();
    builder.format(|buf, record| writeln!(buf, "[{}][{}][{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.target(),
                    record.args()))
            .filter(None, LevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init();
}
