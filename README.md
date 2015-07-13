# lsha = ls + sha [<img src="https://travis-ci.org/kuking/lsha.svg?branch=master">](https://travis-ci.org/kuking/lsha)

Small tool built mostly to learn and practice [Rust language](http://www.rust-lang.org/).

Its purpose is to calculate a sha-256 hash out of folders and optionally file contents. It calculates a checksum based on file names, size, attributes, timestamps and directories (the last two optional). It might be useful to verify folders have not changed or the copies are similar between boxes.

## Usage

```
$ lsha --help
Usage: lsha [options] <PATH>
       lsha --help`
       lsha --version

Options: -c   Checksum file contents
         -r   Recursive
         -t   Use timestamps in checksum
         -q   quiet (don't output file details)
```

## Notice
This project is a trial; its only intention is to help me to practice a bit of rust on my free time; contributions are welcome but please don't expect this to be an example of how you should do Rust programming. Please, don't even expect it to work as intended =)
