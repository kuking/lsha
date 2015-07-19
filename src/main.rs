extern crate crypto;
extern crate docopt;
extern crate libc;

mod simpledir;
use simpledir::SimpleDir;

mod runconfig;
use runconfig::LshaRunConfig;

use docopt::Docopt;
use std::{io, fs};

use crypto::digest::Digest;
use crypto::sha2::Sha256;


fn put(sh : &mut Sha256, cfg :&LshaRunConfig, st : &String) {
    if !cfg.be_quiet {
        print!("{}", st);
    }
    sh.input_str(&st);
}

fn do_path(sh : &mut Sha256, path :&String, cfg :&LshaRunConfig) -> Result<(), io::Error> {

    if cfg.be_recursive {
        put(sh, cfg, &format!("\n{}\n", path));
    }

    let mut data : Vec<_> = try!(fs::read_dir(path))
        .map( |rde| { SimpleDir::new(rde.unwrap()) } )
        .collect::<Vec<_>>();
    data.sort_by(|a, b| a.fname().cmp(&b.fname()));

    for sd in data.iter() {
        if cfg.incl_hidden || !sd.fname().starts_with('.') {
            let s = sd.dump_as_string();
            if cfg.do_file_checksum && sd.is_regular_file() {
                // TODO: calculate file sha here
                // and append to the text
            }
            put(sh, cfg, &format!("{}\n",&s));
        }
    }

    if cfg.be_recursive {
        for sd in data.iter() {
            if sd.mdata().is_dir() {
                do_path(sh, &(String::new() + &path + &"/" + &sd.fname()), cfg).unwrap();
            }
        }
    }

    return Ok(());
}

static USAGE: &'static str = "
Usage: lsha [options] <PATH>
       lsha --help
       lsha --version

Options: -c   Checksum file contents
         -r   Recursive
         -t   Use timestamps in checksum
         -l   Include hidden files
         -q   quiet (don't output file details)
";

fn main() {

    let args = Docopt::new(USAGE).unwrap().parse()
                  .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version") {
        println!("lsha version 0.1");
        return;
    }
    let cfg = LshaRunConfig::from_docopt (args);

    let mut sh = Sha256::new();
    match do_path(&mut sh, &cfg.path, &cfg) {
        Ok(_)  => println!("lsha is {}", sh.result_str()),
        Err(e) => println!("error {}", e.to_string()),
    }
}
