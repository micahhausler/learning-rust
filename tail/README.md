# Tail

This is an implemenatation of `tail` in Rust. It supports a configurable number of lines and filename to read.

```
$ ./target/debug/tail -h
Usage: tail [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]  The filename to read

Options:
  -n, --number <NUMBER>  Number of lines to read [default: 10]
  -h, --help             Print help information
  -V, --version          Print version information
```