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


#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate screencat;

use screencat::errors::*;
use std::env;
use std::process::exit;

fn main() {
    if let Err(ref e) = run() {
        error!("{}", e);

        for e in e.iter().skip(1) {
            error!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            error!("backtrace: {:?}", backtrace);
        }

        exit(1);
    }
}

fn run() -> Result<()> {
    let args = init_cli();
    screencat::init_logging();

    debug!("env var sanity check:");
    for (key, value) in env::vars() {
        debug!("{} = {}", key, value);
    }

    debug!("args: {:?}", args);

    let watch_path = args.value_of("watch_dir").unwrap();
    let regex = args.value_of("regex").unwrap();

    let watcher = screencat::fs_watcher::FSWatcher::new(watch_path, regex, 500)?;
    watcher.start()
}

fn init_cli<'a>() -> clap::ArgMatches<'a> {
    app_from_crate!()
        .arg(clap::Arg::with_name("watch_dir")
            .short("w")
            .long("watch-dir")
            .takes_value(true)
            .default_value("~/Desktop")
            .help("Directory to watch for FS events"))
        .arg(clap::Arg::with_name("regex")
            .short("r")
            .long("regex")
            .takes_value(true)
            .default_value(r"Screen Shot .* at .*\.png$")
            .help("Regex to match FS events again for screenshots"))
    .get_matches()
}
