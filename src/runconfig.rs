
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

    pub fn resolve_arguments_or_exit_with_help() -> LshaRunConfig {

        let args = docopt::Docopt::new(USAGE).unwrap().parse()
                  .unwrap_or_else(|e| e.exit());

        if args.get_bool("--version") {
            println!("lsha version 0.1");
            exit(-1);
        }

        return LshaRunConfig::from_docopt (args);
    }

}
