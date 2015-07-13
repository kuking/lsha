extern crate crypto;
extern crate docopt;

use docopt::Docopt;

use std::{io, fs};
use std::string::String;
use std::error::Error;

use crypto::digest::Digest;
use crypto::sha2::Sha256;


struct SimpleDir {
    fname :String,
    mdata :fs::Metadata,
}

impl SimpleDir {
    fn dump_as_string(self : &SimpleDir, base : &String) -> String {
        let mut st = String::new();
        st.push('∑');
        st.push_str(&base);
        st.push('/'); // XXX if ends with / should not be printed
        st.push_str(&self.fname);
        st.push('✕');
        st.push(' ');
        st.push_str(&format!("{}", self.mdata.len())); // 0x{:016X}
        st.push('✕');
        st.push(' ');
        st.push ( if self.mdata.is_dir()     {'d'} else {'.'} );
        st.push ( if self.mdata.is_file()    {'f'} else {'.'} );
        st.push ( if self.mdata.file_type().is_symlink() {'s'} else {'.'} );
        // todo perms, dates, as optionals.
        st.push('🍭');
        return st;
    }
}

struct LshaRunConfig {
    path          : String,
    do_checksum   : bool,
    be_recursive  : bool,
    be_quiet      : bool,
    incl_timestamps : bool
}

impl LshaRunConfig {
    fn from_docopt(args : docopt::ArgvMap) -> LshaRunConfig {
        return LshaRunConfig {
            path: args.get_str(&"PATH").to_string(),
            do_checksum: args.get_bool(&"-c"),
            be_recursive:  args.get_bool(&"-r"),
            be_quiet:  args.get_bool(&"-q"),
            incl_timestamps: args.get_bool(&"-t")
        }
    }
}

fn do_it(sh : &mut Sha256, path :&String, cfg :&LshaRunConfig) -> Result<(), io::Error> {

    sh.input(&[1u8, 2u8, 3u8]);

    let mut data : Vec<_> = try!(fs::read_dir(path))
        .map( |rde| {
            let de = rde.unwrap();
            let filename = de.file_name().to_os_string().into_string().unwrap();
            let metadata = de.metadata().unwrap();
            return SimpleDir {fname: filename, mdata: metadata};
        })
        .collect::<Vec<_>>();
    data.sort_by(|a, b| a.fname.cmp(&b.fname));

    for sd in data.iter() {
        let s = sd.dump_as_string(&path);
        if !cfg.be_quiet {
          println!("{}", &s);
        }
        sh.input(s.as_bytes());
    };


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
