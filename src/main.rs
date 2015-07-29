extern crate crypto;
extern crate libc;
extern crate docopt;

mod simpledir;
use simpledir::SimpleDir;

mod runconfig;
use runconfig::LshaRunConfig;

use std::{io, fs, iter};
use std::io::Read;
use std::env::args;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha2::Sha256;


fn put(sh : &mut Sha256, cfg :&LshaRunConfig, st : &String) {
    if !cfg.be_quiet {
        print!("{}", st);
    }
    sh.input_str(&st);
}

fn do_file_hash(path :&Path) -> String {

    let mut sh = Sha256::new();
    let ref mut buf = [0; 64*1024];

    if let Ok(mut f) = fs::File::open(path) {
        loop {
            let mut consumed : usize = 0;
            match f.read(buf) {
                Ok(n)  => consumed = n,
                Err(e) => println!("error hashing file {:?}", e)
            }
            if consumed == 0 { break; }
            sh.input(&buf[0..consumed]);
        }
        return sh.result_str();
    } else {
        return "N/A".to_string();
    }
}

fn do_path(sh : &mut Sha256, path :&Path, cfg :&LshaRunConfig) -> Result<(), io::Error> {

    if cfg.be_recursive {
        put(sh, cfg, &format!("\n{}\n", path.display()));
    }

    let mut data : Vec<_> = try!(fs::read_dir(path))
        .map( |rde| { SimpleDir::new(rde.unwrap()) } )
        .collect::<Vec<_>>();
    data.sort_by(|a, b| a.fname().cmp(&b.fname()));

    for sd in data.iter() {

        if !cfg.incl_hidden && sd.is_hidden() {
            continue;
        }

        let s = sd.dump_as_string();
        if cfg.do_file_checksum {
            let hash : String;
            if sd.is_regular_file() {
                hash = do_file_hash(sd.append_fname_to(&path).as_path());
            } else {
                hash = iter::repeat(".   ").take(16).collect();
            }
            put(sh, cfg, &format!("{:64} {}\n", &hash, &s));
        } else {
            put(sh, cfg, &format!("{}\n",&s));
        }

    }

    if cfg.be_recursive {
        for sd in data.iter() {
            if sd.is_dir() && (cfg.incl_hidden || !sd.is_hidden()) {
                let new_path = sd.append_fname_to(&path);
                if let Err(_) = do_path(sh, new_path.as_path(), cfg) {
                   println!("Failure exploring path {:?}", new_path);
                }
            }
        }
    }

    return Ok(());
}

fn main() {
    let cfg = LshaRunConfig::parse_args_or_exit_with_help(args());
    let mut sh = Sha256::new();
    match do_path(&mut sh, &cfg.path, &cfg) {
        Ok(_)  => println!("{} is lsha for {}", sh.result_str(), &cfg.path.display()),
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
