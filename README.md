# mozlz4
Decompress / compress mozlz4 files, specifically search.json.mozlz4 in Firefox.

Binaries available [here](https://github.com/jusw85/mozlz4/releases)

## Usage

    $ ./mozlz4 --help
    mozlz4 0.1.0
    Justin Wong
    Decompress and compress mozlz4 files. Overwrites existing files.

    USAGE:
        mozlz4.exe [FLAGS] <input> [output]

    FLAGS:
        -x, --extract     decompress mozlz4 (default)
        -z, --compress    compress to mozlz4
        -h, --help        Prints help information
        -V, --version     Prints version information

    ARGS:
        <input>     input file, - for stdin
        <output>    output file, - for stdout (default)

## Examples

Extract search.json.lz4

    $ ./mozlz4 search.json.mozlz4 search.json

Compress search.json

    $ ./mozlz4 -z search.json search.json.mozlz4

## Building

    $ cargo build --release

## Notes

    $ rustc --version
    rustc 1.24.0 (4d90ac38c 2018-02-12)

[lz4.h](https://github.com/mozilla/gecko-dev/blob/77a9b22fbc7353b598c8f975f6e9c08d1767133d/mfbt/lz4.h) and [lz4.c](https://github.com/mozilla/gecko-dev/blob/77a9b22fbc7353b598c8f975f6e9c08d1767133d/mfbt/lz4.c) copied directly from Mozilla repo
