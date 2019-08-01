use std::alloc::{alloc_zeroed, Layout};
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::stdin;
use std::os::unix::fs::OpenOptionsExt;
use std::process::exit;
use std::slice;

fn main() {
    let file = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: dwrite <disk> (block)");
        exit(64)
    });

    let block = env::args()
        .nth(2)
        .map(|arg| {
            arg.parse().unwrap_or_else(|_| {
                eprintln!("invalid block size");
                exit(64)
            })
        })
        .unwrap_or(128 * 1024); // default stolen from GNU cat

    let mut output = OpenOptions::new()
        .write(true)
        .custom_flags(libc::O_DIRECT)
        .open(file)
        .unwrap_or_else(|err| {
            eprintln!("file could not be opened: {}", err);
            exit(74)
        });

    let buf_mem = unsafe { alloc_zeroed(Layout::from_size_align(block, 512).unwrap()) };
    let buf = unsafe { slice::from_raw_parts_mut(buf_mem, block) };
    let mut stdin_buf = stdin();
    let mut read;

    loop {
        read = stdin_buf.read(&mut buf[..]).unwrap_or_else(|err| {
            eprintln!("error reading from stdin: {}", err);
            exit(74)
        });

        if read == 0 {
            break;
        }

        for elem in (&mut buf[read..]).iter_mut() {
            *elem = 0;
        }

        output.write(&buf[..]).unwrap_or_else(|err| {
            eprintln!("error writing to output: {}", err);
            exit(74)
        });
    }
}
