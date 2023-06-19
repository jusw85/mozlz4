extern crate byteorder;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

use std::fs::File;
use std::io::prelude::*;
use errors::*;
use clap::{App, Arg};

fn main() {
    if let Err(ref e) = run() {
        use error_chain::ChainedError;
        eprintln!("{}", e.display_chain());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = App::new("mozlz4")
        .version(crate_version!())
        .author("Justin Wong")
        .about("Decompress and compress mozlz4 files. Overwrites existing files.")
        .arg(
            Arg::with_name("decompress")
                .help("decompress mozlz4 (default)")
                .short("x")
                .long("extract")
                .conflicts_with("compress")
                .display_order(1),
        )
        .arg(
            Arg::with_name("compress")
                .help("compress to mozlz4")
                .short("z")
                .long("compress"),
        )
        .arg(
            Arg::with_name("input")
                .help("input file, - for stdin")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("output file, - for stdout (default)")
                .index(2),
        )
        .get_matches();

    let ifilename = matches.value_of("input").unwrap();
    let ofilename = matches.value_of("output").unwrap_or("-");
    let do_compress = matches.is_present("compress");

    let ibuffer = read_to_buffer(ifilename)?;

    let obuffer = if !do_compress {
        mozlz4::decompress(ibuffer).chain_err(|| format!("Failed to decompress"))?
    } else {
        mozlz4::compress(ibuffer).chain_err(|| format!("Failed to compress"))?
    };
    write_to_file(obuffer, ofilename)?;

    Ok(())
}

fn write_to_file(obuffer: Vec<u8>, ofilename: &str) -> Result<()> {
    let mut ofile: Box<dyn Write> = if ofilename == "-" {
        Box::new(std::io::stdout())
    } else {
        Box::new(File::create(ofilename)
            .chain_err(|| format!("Unable to create file {}", ofilename))?)
    };

    ofile
        .write_all(&obuffer[..])
        .chain_err(|| format!("Unable to write to file {}", ofilename))?;
    Ok(())
}

fn read_to_buffer(ifilename: &str) -> Result<Vec<u8>> {
    let mut ifile: Box<dyn Read> = if ifilename == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(ifilename).chain_err(|| format!("Unable to open file {}", ifilename))?)
    };

    let mut ibuffer: Vec<u8> = Vec::new();
    let bytes_read = ifile
        .read_to_end(&mut ibuffer)
        .chain_err(|| format!("Unable to read file {}", ifilename))?;
    assert_eq!(ibuffer.len(), bytes_read);
    Ok(ibuffer)
}
