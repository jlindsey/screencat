Screencat
[![Build Status](https://travis-ci.org/jlindsey/screencat.svg?branch=master)](https://travis-ci.org/jlindsey/screencat)
=========

Screenshot sharing tool using the excellent [Sendcat](https://sendcat.com/) service.

Requirements
------------

This tool requires at least Sendcat v0.0.10, or any later version with the `-j` flag
for JSON output.

Usage
-----

If you are on macOS and haven't fiddled with any of the default screenshot settings, Screencat should
Just Work™. You can place it on your `PATH` and run it with the defaults (or, eg. using a Launch Agent or
in a background `tmux`).

It has two configurable settings via CLI flags: the directory it watches, and the regex to match filesystem
events as screenshots. For example:

```bash
$ /usr/local/bin/screencat -w /tmp/screenshots -r "screenshot-file-.*\.jpg$"
```

License
-------
Copyright (C) 2018  Josh Lindsey <joshua.s.lindsey@gmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
