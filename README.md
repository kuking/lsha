# lsha = ls + sha [<img src="https://travis-ci.org/kuking/lsha.svg?branch=master">](https://travis-ci.org/kuking/lsha) [![Coverage Status](https://coveralls.io/repos/kuking/lsha/badge.svg?branch=master&service=github)](https://coveralls.io/github/kuking/lsha?branch=master)

Small tool built mostly to learn and practice [Rust language](http://www.rust-lang.org/).

Its purpose is to calculate a sha-256 hash out of folders and optionally file contents. It calculates a checksum based on file names, size, attributes, timestamps and directories (the last two optional). It might be useful to verify folders have not changed or the copies are similar between boxes.

## Usage

```
$ lsha --help
Usage: lsha [options] <PATH>
       lsha --help
       lsha --version

Options: -c   Checksum file contents
         -r   Recursive
         -t   Use timestamps in checksum
         -l   Include hidden files
         -q   quiet (don't output file details)
```

* sha256 will be very slow when running in debug mode.

### Output
```
$ lsha -rc .
.
4a974123639ddb638b09682115162fe86a9f28da787be657f5ceebb69a634cc3 -rw-r--r--  501   20      4027 Cargo.lock
9576fbc375f842a6455943f728936b22bf41436e37eaa0aeb2b0a44362c85b6c -rw-r--r--  501   20       174 Cargo.toml
fc0d213f089272d5de0c28f78803dfcd1f56f8f952c7bec298b7fb442b8b919c -rw-r--r--  501   20     35142 LICENSE
[truncated]

./target/debug/lsha.dSYM/Contents/Resources
.   .   .   .   .   .   .   .   .   .   .   .   .   .   .   .    drwxr-xr-x  501   20       102 DWARF

./target/debug/lsha.dSYM/Contents/Resources/DWARF
26c9d3a570ff3f03dc61049854dfd6599f63682c70e310310ccf534338b790be -rw-r--r--  501   20   3366242 lsha

94d695beb71d7678077bbb9aa0aaaf3260816f53a0b45ede48b728f0fa1af7b2 is lsha for .

```


## Notice
This project is a trial; its only intention is to help me to practice a bit of rust on my free time; contributions are welcome but please don't expect this to be an example of how you should do Rust programming. Please, don't even expect it to work as intended =)
