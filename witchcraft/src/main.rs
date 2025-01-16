use core::{consts::*, core::*};
use std::process;

use modules::{
    binds::binds::{self, flawless_entry_point},
    blackcat::blackcat,
    network::network,
    osint::osint,
    social::social,
    tldr::tldr,
};

mod core;
mod modules;

fn main() {
    let argsv = readargs();
    if argsv.len() % 2 != 0 {
        println!("{}", WITCH);
        process::exit(42);
    }

    let arg_name = argsv[1].as_str();

    let mut join_closures = Vec::new();
    join_closures.extend(binds::api());
    join_closures.extend(blackcat::api());
    join_closures.extend(network::api());
    join_closures.extend(osint::api());
    join_closures.extend(social::api());
    join_closures.extend(tldr::api());

    if arg_name == "help" || arg_name == "h" {
        magic_docs();
    }

    if arg_name == "manual" || arg_name == "h" {
        raise(MAN_HEADER, "");
        magic_docs();
    }

    if arg_name == "version" || arg_name == "v" {
        show_version();
    }

    let code = closure_shell(join_closures, &argsv);
    if code == 11223300 {
        let code = flawless_entry_point(&argsv);
        process::exit(code);
    }

    process::exit(code);
}

#[cfg(test)]
mod test;
