
use docopt;
use std::process::exit;
use std::path::PathBuf;

static VERSION: &'static str =
    "lsha version 0.1 originally by Eduardo ES Riccardi (https://github.com/kuking/lsha)";

static USAGE: &'static str = "
Usage: lsha [options] <PATH>
       lsha (-h | --help)
       lsha --version

Options: -c   Checksum file contents
         -r   Recursive
         -t   Use timestamps in checksum
         -l   Include hidden files
         -q   quiet (don't output file details)
  --help -h   Shows help screen
";

#[derive(Debug)]
pub struct LshaRunConfig {
    pub path           : PathBuf,
    pub do_file_checksum : bool,
    pub be_recursive    : bool,
    pub be_quiet        : bool,
    pub incl_timestamps : bool,
    pub incl_hidden     : bool
}

impl LshaRunConfig {

    fn from_docopt(args : docopt::ArgvMap) -> LshaRunConfig {
        return LshaRunConfig {
            path:               PathBuf::from(args.get_str(&"PATH")),
            do_file_checksum:   args.get_bool(&"-c"),
            be_recursive:       args.get_bool(&"-r"),
            be_quiet:           args.get_bool(&"-q"),
            incl_timestamps:    args.get_bool(&"-t"),
            incl_hidden:        args.get_bool(&"-l")
        }
    }

    pub fn parse_args_or_exit_with_help<I, S>(argv : I) -> LshaRunConfig
     where I: Iterator<Item=S>, S: AsRef<str> {

        let args = docopt::Docopt::new(USAGE)
               .and_then(|d| d.argv(argv.into_iter()).parse())
               .unwrap_or_else(|e| e.exit());

        if args.get_bool(&"--version") {
            println!("{}", &VERSION);
            exit(-1);
        }

        return LshaRunConfig::from_docopt (args);
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use docopt;

    fn given_docopt(line : &str) -> docopt::ArgvMap {
        return docopt::Docopt::new(super::USAGE)
                        .unwrap()
                        .argv(line.split(' ').into_iter())
                        .parse()
                        .unwrap_or_else(|e|e.exit());
    }

    #[test]
    fn no_params() {
        let cfg = LshaRunConfig::from_docopt(given_docopt(&"lsha ."));

        assert!(!cfg.do_file_checksum);
        assert!(!cfg.be_recursive);
        assert!(!cfg.be_quiet);
        assert!(!cfg.incl_timestamps);
        assert!(!cfg.incl_hidden);
        assert_eq!(".", cfg.path.to_str().unwrap());
    }

    #[test]
    fn all_params() {
        let cfg = LshaRunConfig::from_docopt(given_docopt(&"lsha -qcrtl le-path"));

        assert!(true, cfg.do_file_checksum);
        assert!(true, cfg.be_recursive);
        assert!(true, cfg.be_quiet);
        assert!(true, cfg.incl_timestamps);
        assert!(true, cfg.incl_hidden);
        assert_eq!("le-path", cfg.path.to_str().unwrap());
    }

    #[test]
    fn mix_params() {
        let cfg = LshaRunConfig::from_docopt(given_docopt(&"lsha -q -l path"));

        assert!(!cfg.do_file_checksum);
        assert!(!cfg.be_recursive);
        assert!(cfg.be_quiet);
        assert!(!cfg.incl_timestamps);
        assert!(cfg.incl_hidden);
        assert_eq!("path", cfg.path.to_str().unwrap());
    }

    //#[test]
    fn version_should_finish() {
        let cfg = LshaRunConfig::from_docopt(given_docopt(&"lsha --version"));
        panic!("meh");
    }
}
