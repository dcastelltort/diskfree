#[macro_use] 
extern crate quicli;
use quicli::prelude::*;

extern crate libc;

extern crate ansi_term;
use ansi_term::Colour::{Red, Yellow, Green};

mod diskspace;
use diskspace::*;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "h", long = "--human-readable", help = "Print sizes in human readable format (e.g., 1K 234M 2G).")]
    human_readable_base2: bool,
    #[structopt(short = "H", long = "--si", help = "Same as -h, but use powers of 1000 instead of 1024.")]
    human_readable_base10: bool,
    mount_point: String,
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

fn format_capacity(capacity: u8) -> String {
    let formatted = format!("{}%", capacity);
    if capacity < 20 {
        Red.paint(formatted).to_string()
    }
    else if capacity < 30 {
        Yellow.paint(formatted).to_string()
    }
    else if capacity > 50 {
        Green.paint(formatted).to_string()
    } else {
        formatted
    }
}
main!(|args: Cli, log_level: verbosity| {
    let stats = diskspace(&args.mount_point).unwrap();
    
    let base = if args.human_readable_base2 { HUMAN_BASE_1024 } 
                else if args.human_readable_base10 { HUMAN_BASE_1000 } 
                else { HUMAN_BASE };

    println!("Size {}, Avail {}, Capacity {}", human_readable_size(stats.used, base), 
                                                human_readable_size(stats.avail, base) , 
                                                format_capacity(stats.capacity));

});
