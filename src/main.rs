extern crate crypto;
extern crate docopt;
extern crate libc;

use docopt::Docopt;

use std::{io, fs};
use std::fs::Metadata;

use libc::consts::os::posix88::*;

// the following makes the whole thing unix only
use std::os::unix::fs::MetadataExt;


use std::string::String;
use std::error::Error;

use crypto::digest::Digest;
use crypto::sha2::Sha256;


struct SimpleDir {
    fname :String,
    mdata :fs::Metadata,
}

impl SimpleDir {

    fn mode_as_string(mode :&u16) -> String {
        fn rwx(mode :&u16, rm :u16, wm :u16, xm :u16) -> String {
            let mut st = String::new();
            if mode & rm == rm { st.push('r') } else { st.push('-') };
            if mode & wm == wm { st.push('w') } else { st.push('-') };
            if mode & xm == xm { st.push('x') } else { st.push('-') };
            return st;
        }
        fn dbpcs(mode :&u16) -> char {
            if      mode & S_IFLNK == S_IFLNK { return 's' }
            else if mode & S_IFREG == S_IFREG { return '-' }
            else if mode & S_IFBLK == S_IFBLK { return 'b' }
            else if mode & S_IFDIR == S_IFDIR { return 'd' }
            else if mode & S_IFIFO == S_IFIFO { return 'p' }
            else if mode & S_IFCHR == S_IFCHR { return 'c' }
            else { return '?' }
        }
        return format!("{}{}{}{}", dbpcs(mode),
                                 rwx(mode, S_IRUSR, S_IWUSR, S_IXUSR),
                                 rwx(mode, S_IRGRP, S_IWGRP, S_IXGRP),
                                 rwx(mode, S_IROTH, S_IWOTH, S_IXOTH)).to_string();
    }

    fn dump_as_string(self : &SimpleDir) -> String {
        let mut st = String::new();
        let mode = self.mdata.mode();
        st = st + &SimpleDir::mode_as_string(&mode);
        st = st + &format!("{:10} ", &self.mdata.len());
        st = st + & self.fname;
        return st;
    }
    fn new(de : std::fs::DirEntry) -> SimpleDir {
        SimpleDir {
            fname: de.file_name().to_os_string().into_string().unwrap(),
            mdata: de.metadata().unwrap()
        }
    }
}

#[test]
fn mode_as_string_tests() {
    assert_eq!("?---------", SimpleDir::mode_as_string(&0));
    assert_eq!("?rwx------", SimpleDir::mode_as_string(&(S_IRUSR+S_IWUSR+S_IXUSR)));
    assert_eq!("?---rwx---", SimpleDir::mode_as_string(&(S_IRGRP+S_IWGRP+S_IXGRP)));
    assert_eq!("?------rwx", SimpleDir::mode_as_string(&(S_IROTH+S_IWOTH+S_IXOTH)));
    assert_eq!("?r--r--r--", SimpleDir::mode_as_string(&(S_IRUSR+S_IRGRP+S_IROTH)));
    assert_eq!("?-w--w--w-", SimpleDir::mode_as_string(&(S_IWUSR+S_IWGRP+S_IWOTH)));
    assert_eq!("?--x--x--x", SimpleDir::mode_as_string(&(S_IXUSR+S_IXGRP+S_IXOTH)));
    assert_eq!("d---------", SimpleDir::mode_as_string(&S_IFDIR));
    assert_eq!("b---------", SimpleDir::mode_as_string(&S_IFBLK));
    assert_eq!("p---------", SimpleDir::mode_as_string(&S_IFIFO));
    assert_eq!("c---------", SimpleDir::mode_as_string(&S_IFCHR));
    assert_eq!("s---------", SimpleDir::mode_as_string(&S_IFLNK));
    assert_eq!("drwxrwxrwx", SimpleDir::mode_as_string(
      &(S_IFDIR+S_IRUSR+S_IWUSR+S_IXUSR+S_IRGRP+S_IWGRP+S_IXGRP+S_IROTH+S_IWOTH+S_IXOTH)));
}



struct LshaRunConfig {
    path           : String,
    do_file_checksum : bool,
    be_recursive    : bool,
    be_quiet        : bool,
    incl_timestamps : bool
}

impl LshaRunConfig {
    fn from_docopt(args : docopt::ArgvMap) -> LshaRunConfig {
        return LshaRunConfig {
            path: args.get_str(&"PATH").to_string(),
            do_file_checksum: args.get_bool(&"-c"),
            be_recursive:  args.get_bool(&"-r"),
            be_quiet:  args.get_bool(&"-q"),
            incl_timestamps: args.get_bool(&"-t")
        }
    }
}

fn do_it(sh : &mut Sha256, path :&String, cfg :&LshaRunConfig) -> Result<(), io::Error> {

    let mut data : Vec<_> = try!(fs::read_dir(path))
        .map( |rde| { SimpleDir::new(rde.unwrap()) } )
        .collect::<Vec<_>>();
    data.sort_by(|a, b| a.fname.cmp(&b.fname));

    for sd in data.iter() {
        let s = sd.dump_as_string();
        if !cfg.be_quiet {
          println!("{}", &s);
        }
        sh.input_str(&s);
        sh.input_str(&"\n");

    }

    if cfg.be_recursive {
        for sd in data.iter() {
            if sd.mdata.is_dir() {
                let mut temp = String::new();
                temp.push_str(path);
                temp.push('/');
                temp.push_str(&sd.fname);
                do_it(sh, &temp, cfg).unwrap();
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
    match do_it(&mut sh, &cfg.path, &cfg) {
        Ok(_)  => println!("lsha-256 {}", sh.result_str()),
        Err(e) => println!("error {}", e.to_string()),
    }
}
