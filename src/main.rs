extern crate clap;
use clap::{Arg, App};
use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;
use core::fmt::Write;
use unicode_segmentation::UnicodeSegmentation;
// use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let matches = App::new("utf8-stats")
                          .version("1.0")
                          .author("nomorethirst <nomorethirst@users.noreply.github.com>")
                          .about("Prints statistics of utf8 graphemes, chars, charsets, etc.")
                          .arg(Arg::with_name("FILENAME")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();
    
    let filename = matches.value_of("FILENAME").unwrap();
    let (v, vv, vvv) = set_verbosity(matches.occurrences_of("v"));

    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     println!("Are you missing the filename argument?");
    //     println!("usage: utf8-stats <filename>");
    //     exit(1);
    // }
    // let path = Path::new(&args[1]);
    // let contents = match fs::read_to_string(path) {
    //     Err(e) => {
    //         println!("error - could not open {}", path.display());
    //         println!("{}\nusage: utf8-stats <filename>", e);
    //         exit(1);
    //     },
    //     Ok(f) => f,
    // };
    let contents = get_file_contents(filename);
    let verbose: bool = false;

    let mut grapheme_counts: HashMap<&str, u32> = HashMap::new();
    let mut num_graphemes: u32 = 0;
    let mut num_chars: u32 = 0;

    if vvv {
        println!("     index g utf8      unicode          unicode_expanded");
    }
    for g in contents.grapheme_indices(true) {
        
        *grapheme_counts.entry(g.1).or_insert(0) += 1;
        num_graphemes += 1;
        
        if vvv {
            // print index and grapheme
            print!("{:>10} {}", g.0, g.1);

            // print utf8 bytes of grapheme in hex w/ spaces and fixed-width
            let hexstrlen = 3 * g.1.len();
            let mut s = String::with_capacity(hexstrlen);
            for b in g.1.bytes() {
                write!(s, " {:02X}", b);
            }
            print!("{:<10}", s);

            //print escaped unicode
            let ustr = g.1.escape_unicode().to_string().to_uppercase();
            print!(" {:<16}", ustr);
        }

        for c in g.1.chars() {
            num_chars += 1;
            if vvv {
                //print expanded unicode - char, hex, dec for each unicode scalar
                let cint: u32 = c as u32;
                print!(" [{} {:0>8X} {:>8}]", c, cint, cint);
            }
        }
        if vvv { println!(); }
    }

    println!("total graphemes: {}", num_graphemes);
    println!("total chars: {}", num_chars);
    println!("unique graphemes: {}", grapheme_counts.len());

    if v {
        // for (k, v) in grapheme_counts.iter() {
        //     println!("{}: {}", k, v);
        // }
        // collect grapheme counts into vector and reverse sort (by highest frequency)
        let mut grapheme_counts_sorted: Vec<_> = grapheme_counts.iter().collect();
        // grapheme_counts_sorted.sort_by(|a, b| a.1.cmp(b.1).reverse()); // by count desc
        grapheme_counts_sorted.sort_by(|a, b| a.0.cmp(b.0)); // by grapheme
        for (k, v) in grapheme_counts_sorted {
            println!("{}: {}", k, v);
        }
    }
    // println!("{}", contents);
}

fn get_file_contents(filename: &str) -> String {
    let path = Path::new(filename);
    let contents = match fs::read_to_string(path) {
        Err(e) => {
            println!("error - could not open {}", path.display());
            println!("{}\nusage: utf8-stats <filename>", e);
            exit(1);
        },
        Ok(f) => f,
    };
    return contents;
}

fn set_verbosity(num_v: u64) -> (bool, bool, bool) {
    match num_v {
        0 => return (false, false, false),
        1 => return (true, false, false),
        2 => return (true, true, false),
        3 | _ => return (true, true, true),
    }
}
