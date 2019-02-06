// flatkvm-paste
// Copyright (C) 2019  Sergio Lopez <slp@sinrega.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use clap::{crate_authors, crate_version, App, Arg};
use std::io::{stdin, Read};
use x11_clipboard::Clipboard;

fn main() {
    let cmd_args = App::new("flatkvm-paste")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Paste text into a Flatkvm session.")
        .arg(
            Arg::with_name("pid")
                .required(true)
                .help("PID of the Flatkvm process managing the session"),
        )
        .get_matches();

    let pid: u32 = match cmd_args.value_of("pid") {
        Some(pid) => pid.parse().unwrap(),
        None => panic!("invalid arguments"),
    };

    let mut data = Vec::new();
    stdin().read_to_end(&mut data).expect("error reading from stdin");

    let clipboard = Clipboard::new().unwrap();
    let name = format!("FK{}", pid);
    let selection = clipboard.setter.get_atom(&name).unwrap();
    clipboard
        .store_wait(selection, clipboard.setter.atoms.utf8_string, data)
        .unwrap();
}
