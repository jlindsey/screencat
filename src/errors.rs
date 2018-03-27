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

use ::std::io;
use ::std::num;
use ::std::sync::mpsc;
use ::notify;
use ::regex;
use ::serde_json;

error_chain! {
    foreign_links {
        IO(io::Error);
        Notify(notify::Error);
        Parse(num::ParseIntError);
        RecvErr(mpsc::RecvError);
        Regex(regex::Error);
        JSON(serde_json::Error);
    }

    errors {
        ClipboardError(t: String) {
            description("error manipulating system clipboard")
            display("error manipulating system clipboard: {}", t)
        }
    }
}
