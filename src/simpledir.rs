
use libc::consts::os::posix88::*;
use libc::types::os::arch::posix88::mode_t;

use std::fs::DirEntry;
use std::fs::Metadata;

use std::os::unix::fs::MetadataExt;


pub struct SimpleDir {
    fname :String,
    mdata :Metadata,
}

impl SimpleDir {

    pub fn mdata(&self) -> &Metadata {
        return &self.mdata;
    }

    pub fn fname(&self) -> &String {
        return &self.fname;
    }

    fn mode_as_string(mode :&mode_t) -> String {
        fn rwx(mode :&mode_t, rm :mode_t, wm :mode_t, xm :mode_t) -> String {
            let mut st = String::new();
            if mode & rm == rm { st.push('r') } else { st.push('-') };
            if mode & wm == wm { st.push('w') } else { st.push('-') };
            if mode & xm == xm { st.push('x') } else { st.push('-') };
            return st;
        }
        fn dbpcs(mode :&mode_t) -> char {
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

    pub fn dump_as_string(self : &SimpleDir) -> String {
        let mut st = String::new();

        st = st + &format!("{}{:5}{:5}",
                            &SimpleDir::mode_as_string(&self.mdata().mode()),
                            &self.mdata().uid(),
                            &self.mdata().gid());


        st = st + &format!("{:10} ", &self.mdata().len());
        st = st + & self.fname;
        return st;
    }

    pub fn is_regular_file(self :&SimpleDir) -> bool {
        return self.mdata().mode() & S_IFREG == S_IFREG;
    }

    pub fn new(de : DirEntry) -> SimpleDir {
        SimpleDir {
            fname: de.file_name().to_os_string().into_string().unwrap(),
            mdata: de.metadata().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::consts::os::posix88::*;

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
}
