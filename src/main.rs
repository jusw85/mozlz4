extern crate byteorder;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate mozlz4_sys;

mod errors {
    error_chain!{}
}

use std::fs::File;
use std::io::prelude::*;
use errors::*;
use clap::{App, Arg};
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use mozlz4_sys::*;

const MAGIC_NUMBER: &[u8] = b"mozLz40\0";

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
                .short("c")
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
        decompress(ibuffer).chain_err(|| format!("Failed to decompress"))?
    } else {
        compress(ibuffer).chain_err(|| format!("Failed to compress"))?
    };
    write_to_file(obuffer, ofilename)?;

    Ok(())
}

fn write_to_file(obuffer: Vec<u8>, ofilename: &str) -> Result<()> {
    let mut ofile: Box<Write> = if ofilename == "-" {
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

fn read_to_buffer(ifilename: &str) -> Result<(Vec<u8>)> {
    let mut ifile: Box<Read> = if ifilename == "-" {
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

fn decompress(ibuffer: Vec<u8>) -> Result<(Vec<u8>)> {
    let magic_number_len = MAGIC_NUMBER.len();
    if ibuffer.len() < (magic_number_len + 4) || !ibuffer.starts_with(MAGIC_NUMBER) {
        bail!("Unrecognized input file")
    }

    let decompressed_size = ibuffer
        .get(magic_number_len..(magic_number_len + 4))
        .unwrap();
    let decompressed_size = LittleEndian::read_u32(decompressed_size) as usize;

    let block = ibuffer.get(magic_number_len + 4..).unwrap();
    let mut obuffer: Vec<u8> = Vec::with_capacity(decompressed_size);

    unsafe {
        let bytes_decompressed = LZ4_decompress_safe(
            block.as_ptr() as *const i8,
            obuffer.as_mut_ptr() as *mut i8,
            block.len() as i32,
            decompressed_size as i32,
        );
        if bytes_decompressed < 0 {
            bail!("Malformed input file")
        }
        obuffer.set_len(bytes_decompressed as usize);
    }
    Ok(obuffer)
}

fn compress(ibuffer: Vec<u8>) -> Result<(Vec<u8>)> {
    let uncompressed_size = ibuffer.len();
    let compress_bound = unsafe { LZ4_compressBound(uncompressed_size as i32) as usize };

    let magic_number_len = MAGIC_NUMBER.len();
    let mut obuffer: Vec<u8> = Vec::with_capacity(magic_number_len + 4 + compress_bound);
    obuffer.extend(MAGIC_NUMBER);
    obuffer
        .write_u32::<LittleEndian>(uncompressed_size as u32)
        .unwrap();

    unsafe {
        let bytes_compressed = LZ4_compress_default(
            ibuffer.as_ptr() as *const i8,
            obuffer[(magic_number_len + 4)..].as_mut_ptr() as *mut i8,
            uncompressed_size as i32,
            compress_bound as i32,
        );
        if bytes_compressed <= 0 {
            bail!("Compression failed")
        }
        obuffer.set_len(magic_number_len + 4 + bytes_compressed as usize);
    }
    Ok((obuffer))
}
