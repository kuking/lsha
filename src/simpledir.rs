
use libc::consts::os::posix88::*;
use libc::types::os::arch::posix88::mode_t;
use libc::types::os::arch::posix88::uid_t;
use libc::types::os::arch::posix88::gid_t;

use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use std::os::unix::fs::MetadataExt;


pub struct SimpleDir {
    fname :String,
    len   :u64,
    mode  :mode_t,
    uid   :uid_t,
    gid   :gid_t
}

impl SimpleDir {

    pub fn fname(&self) -> &String {
        return &self.fname;
    }
    pub fn len(&self) -> u64 {
        return self.len;
    }
    pub fn mode(&self) -> &mode_t {
        return &self.mode;
    }
    pub fn uid(&self) -> &uid_t {
        return &self.uid;
    }
    pub fn gid(&self) -> &gid_t {
        return &self.gid;
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
                            &SimpleDir::mode_as_string(&self.mode()),
                            &self.uid(),
                            &self.gid());


        st = st + &format!("{:10} ", &self.len());
        st = st + & self.fname;
        return st;
    }

    pub fn is_regular_file(self :&SimpleDir) -> bool {
        let mode = self.mode();
        return mode & S_IFREG == S_IFREG
            && mode & S_IFLNK != S_IFLNK
            && mode & S_IFCHR != S_IFCHR
            && mode & S_IFBLK != S_IFBLK
            && mode & S_IFIFO != S_IFIFO
    }

    pub fn is_hidden(self :&SimpleDir) -> bool {
        return self.fname().starts_with('.');
    }

    pub fn is_dir(self :&SimpleDir) -> bool {
        let mode = self.mode();
        return mode & S_IFDIR == S_IFDIR
            && mode & S_IFLNK != S_IFLNK
            && mode & S_IFCHR != S_IFCHR
            && mode & S_IFBLK != S_IFBLK
            && mode & S_IFIFO != S_IFIFO
    }

    pub fn append_fname_to(self :&SimpleDir, path :&Path) -> PathBuf {
        let mut new_path = PathBuf::from(path);
        new_path.push(Path::new(&self.fname()));
        return PathBuf::from(new_path);
    }

    pub fn new(de : DirEntry) -> SimpleDir {
        SimpleDir {
            fname: de.file_name().to_os_string().into_string().unwrap(),
            len  : de.metadata().unwrap().len(),
            mode : de.metadata().unwrap().mode(),
            uid  : de.metadata().unwrap().uid(),
            gid  : de.metadata().unwrap().gid()
        }
    }

    #[cfg(test)]
    fn new_for_test(filename :String, len :u64, mode :mode_t, uid :uid_t, gid :gid_t) -> SimpleDir {
        SimpleDir {
            fname: filename,
            len  : len,
            mode : mode,
            uid  : uid,
            gid  : gid
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::consts::os::posix88::*;
    use std::path::{Path, PathBuf};

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

    #[test]
    fn is_regular_file() {

        // happy
        let sd = SimpleDir::new_for_test("a-file".to_string(), 123, S_IFREG, 0, 0);
        assert!(sd.is_regular_file());

        // a dir is not a regular file
        let sd = SimpleDir::new_for_test("a-dir".to_string(), 123, S_IFDIR, 0, 0);
        assert!(!sd.is_regular_file());

        // file flag plus specialsq are not regular files
        let sd_IFREG_IFLNK =  SimpleDir::new_for_test("file".to_string(), 123, S_IFREG | S_IFLNK, 0, 0);
        let sd_IFREG_IFCHR =  SimpleDir::new_for_test("file".to_string(), 123, S_IFREG | S_IFCHR, 0, 0);
        let sd_IFREG_IFBLK =  SimpleDir::new_for_test("file".to_string(), 123, S_IFREG | S_IFBLK, 0, 0);
        let sd_IFREG_IFIFO =  SimpleDir::new_for_test("file".to_string(), 123, S_IFREG | S_IFIFO, 0, 0);
        assert!(!sd_IFREG_IFLNK.is_regular_file());
        assert!(!sd_IFREG_IFCHR.is_regular_file());
        assert!(!sd_IFREG_IFBLK.is_regular_file());
        assert!(!sd_IFREG_IFIFO.is_regular_file());
    }

    #[test]
    fn it_appends_fname_to() {
        let sd = SimpleDir::new_for_test("file".to_string(), 123, S_IFREG, 0, 0);
        let path = PathBuf::from(&"a-path");
        assert_eq!("a-path", path.to_str().unwrap());
        assert_eq!("file", sd.fname());

        let new_path = sd.append_fname_to(path.as_path());
        assert_eq!("a-path/file", new_path.to_str().unwrap());
    }

    #[test]
    fn is_dir() {

        // happy
        let sd = SimpleDir::new_for_test("dir".to_string(), 123, S_IFDIR, 0, 0);
        assert!(sd.is_dir());

        // regular file is not a dir
        let sd = SimpleDir::new_for_test("dir".to_string(), 123, S_IFREG, 0, 0);
        assert!(!sd.is_dir());

        // dir flag plus specials are not dirs
        let sd_IFDIR_IFLNK =  SimpleDir::new_for_test("dir".to_string(), 123, S_IFDIR | S_IFLNK, 0, 0);
        let sd_IFDIR_IFCHR =  SimpleDir::new_for_test("dir".to_string(), 123, S_IFDIR | S_IFCHR, 0, 0);
        let sd_IFDIR_IFBLK =  SimpleDir::new_for_test("dir".to_string(), 123, S_IFDIR | S_IFBLK, 0, 0);
        let sd_IFDIR_IFIFO =  SimpleDir::new_for_test("dir".to_string(), 123, S_IFDIR | S_IFIFO, 0, 0);
        assert!(!sd_IFDIR_IFLNK.is_dir());
        assert!(!sd_IFDIR_IFCHR.is_dir());
        assert!(!sd_IFDIR_IFBLK.is_dir());
        assert!(!sd_IFDIR_IFIFO.is_dir());
    }
}
