use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use packagemanagers::PackageManager;

pub mod packagemanagers;

// Minor change, but I've replaced String with Box<str> here.
// The main advantage of strings is that you can mutate them. Since that's unnecessary in this case, Box<str> is slightly more efficient.
// Strings allocate the pointer, length, and capacity on the heap, Box<str> doesn't allocate a capacity.
pub struct System {
    pub id: Box<str>,
    pub pretty_name: Box<str>,
    pub package_manager: Option<PackageManager>,
}

impl System {
    pub fn info() -> System {
        let (id, pretty_name) = get_distribution();
        // and_then is similar to Option::map (as mentioned below), but it takes in a closure that returns another Optional value.
        let package_manager =
            get_package_manager(id.as_ref()).and_then(|name| packagemanagers::get(name));

        // let pm: packagemanagers::PackageManager;

        // match packagemanagers::get(package_manager) {
        // Some(value) => pm = value,
        // None => panic!("Could not find a suitable package manager")
        // }
        //
        // Variables are nearly never declared like this in Rust. (Nearly) identical behaviour would be achieved with the expect() function
        // let pm = packagemanagers::get(package_manager).expect("Could not find a suitable package manager");
        //
        // Regardless, we shouldn't be panicking on failure to find a package manager. Instead, let's leave that as an optional value.

        Self {
            id,
            pretty_name,
            package_manager,
        }
    }
}

// Once again, we should return an optional value here. Since the only possible values are known at compile time, we can use a static string slice rather than a string.
// Never accept an immutable borrowed String in function parameters. This creates unnecessary indirection (pointer to a pointer) and you can't do anything extra with it. If you need ownership, take in a String and let the caller handle it.
// This doesn't apply for mutable references, since the length of string slices can't be modified.
fn get_package_manager(distro: &str) -> Option<&'static str> {
    // We shouldn't use a HashMap if we're only indexing into it once. Instead, just use an array.
    let package_managers = [
        ("fedora", "dnf"),
        ("debian", "apt-get"),
        ("arch", "pacman"),
        ("opensuse", "zypper"),
    ];

    package_managers
        .into_iter()
        .find(|(key, _)| key == &distro)
        // Map can be used on options as well, transforming the value into what the closure specifies, if the value is Some. The same applies to Result (Ok).
        .map(|(_, value)| value)
}

fn get_distribution() -> (Box<str>, Box<str>) {
    // The try operator (?) returns a Result or Option, if the value is None or Err (in the case that the err value is or can be transformed into the err type)
    let mut info = get_os_info();

    // Don't allocate a string to index in, we can use a string slice.
    // Since we only need these few values from the map, we can remove them rather than taking a reference, in order to avoid an unnecessary allocation.
    let id = info.remove("id").unwrap_or("unknown".into());
    let pretty_name = info.remove("pretty_name").unwrap_or("unknown".into());

    (id, pretty_name)
}

fn get_os_info() -> HashMap<Box<str>, Box<str>> {
    // os-release existing is a precondition which should be required. Therefore, we'll use expect() to specify why we expect this always to return an Ok value
    let contents = std::fs::read_to_string("/etc/os-release")
        .expect("os-release should exist on all Linux systems.");

    contents
        .lines()
        .filter_map(|line| {
            // Splitting the line once is probably a better idea than creating an iterator, and we should remove quotes in case they're present.
            line.split_once('=')
                .map(|(key, value)| (key.to_lowercase().into(), value.trim_matches('"').into()))
        })
        // The return type is implied, so we don't need to specify it to collect() in this case.
        .collect()
}
