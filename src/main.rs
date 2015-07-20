extern crate crypto;
extern crate libc;
extern crate docopt;

mod simpledir;
use simpledir::SimpleDir;

mod runconfig;
use runconfig::LshaRunConfig;

use std::{io, fs};
use std::env::args;

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
        if cfg.incl_hidden || !sd.is_hidden() {
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
            if sd.mdata().is_dir() && (cfg.incl_hidden || !sd.is_hidden()) {
                do_path(sh, &(String::new() + &path + &"/" + &sd.fname()), cfg).unwrap();
            }
        }
    }

    return Ok(());
}

fn main() {
    let cfg = LshaRunConfig::parse_args_or_exit_with_help(args());
    let mut sh = Sha256::new();
    match do_path(&mut sh, &cfg.path, &cfg) {
        Ok(_)  => println!("lsha({}) = {}", &cfg.path, sh.result_str()),
        Err(e) => println!("error {}", e.to_string()),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn placeholder() {
        // this should test something, needed otherwise coveralls won't pick it
    }

}
