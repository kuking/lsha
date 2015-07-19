extern crate docopt;

pub struct LshaRunConfig {
    pub path           : String,
    pub do_file_checksum : bool,
    pub be_recursive    : bool,
    pub be_quiet        : bool,
    pub incl_timestamps : bool,
    pub incl_hidden     : bool
}

impl LshaRunConfig {
    pub fn from_docopt(args : docopt::ArgvMap) -> LshaRunConfig {
        return LshaRunConfig {
            path: args.get_str(&"PATH").to_string(),
            do_file_checksum: args.get_bool(&"-c"),
            be_recursive:  args.get_bool(&"-r"),
            be_quiet:  args.get_bool(&"-q"),
            incl_timestamps: args.get_bool(&"-t"),
            incl_hidden: args.get_bool(&"-l")
        }
    }
}
