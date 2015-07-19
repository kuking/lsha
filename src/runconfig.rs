
use docopt;
use std::process::exit;

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

pub struct LshaRunConfig {
    pub path           : String,
    pub do_file_checksum : bool,
    pub be_recursive    : bool,
    pub be_quiet        : bool,
    pub incl_timestamps : bool,
    pub incl_hidden     : bool
}

impl LshaRunConfig {

    fn from_docopt(args : docopt::ArgvMap) -> LshaRunConfig {
        return LshaRunConfig {
            path: args.get_str(&"PATH").to_string(),
            do_file_checksum: args.get_bool(&"-c"),
            be_recursive:  args.get_bool(&"-r"),
            be_quiet:  args.get_bool(&"-q"),
            incl_timestamps: args.get_bool(&"-t"),
            incl_hidden: args.get_bool(&"-l")
        }
    }

    pub fn parse_args_or_exit_with_help<I, S>(argv : I) -> LshaRunConfig where I: Iterator<Item=S>, S: Into<String> {

        let args = docopt::Docopt::new(USAGE).unwrap().argv(argv).parse()
                  .unwrap_or_else(|e| e.exit());

        if args.get_bool("--version") {
            println!("lsha version 0.1");
            exit(-1);
        }

        return LshaRunConfig::from_docopt (args);
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use docopt;
    use std::vec::Vec;

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

        assert_eq!(false, cfg.do_file_checksum);
        assert_eq!(false, cfg.be_recursive);
        assert_eq!(false, cfg.be_quiet);
        assert_eq!(false, cfg.incl_timestamps);
        assert_eq!(false, cfg.incl_hidden);
        assert_eq!(".".to_string(), cfg.path);
    }

    #[test]
    fn all_params() {
        let cfg = LshaRunConfig::from_docopt(given_docopt(&"lsha -qcrtl le-path"));

        assert_eq!(true, cfg.do_file_checksum);
        assert_eq!(true, cfg.be_recursive);
        assert_eq!(true, cfg.be_quiet);
        assert_eq!(true, cfg.incl_timestamps);
        assert_eq!(true, cfg.incl_hidden);
        assert_eq!("le-path".to_string(), cfg.path);
    }

    #[test]
    fn mix_params() {
        let cfg = LshaRunConfig::from_docopt(given_docopt(&"lsha -q -l path"));

        assert_eq!(false, cfg.do_file_checksum);
        assert_eq!(false, cfg.be_recursive);
        assert_eq!(true, cfg.be_quiet);
        assert_eq!(false, cfg.incl_timestamps);
        assert_eq!(true, cfg.incl_hidden);
        assert_eq!("path".to_string(), cfg.path);
    }
}
