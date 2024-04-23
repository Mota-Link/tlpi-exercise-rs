use clap::{arg, command};
use libc::{close, open, read, write, O_APPEND, O_CREAT, O_TRUNC, O_WRONLY, S_IWUSR};
use std::ffi::{c_char, c_int, c_void};

const BUF_SIZE: usize = 1024;

fn main() {
    let args = command!()
        .arg(arg!(<NAME>).required(true))
        .arg(arg!(-a --append))
        .get_matches();
    let name = args.get_one::<String>("NAME").unwrap();
    let append = args.get_one::<bool>("append").unwrap();

    let write_flag = if *append { O_APPEND } else { O_TRUNC };
    let flags = O_WRONLY | O_CREAT | write_flag;
    let mode = S_IWUSR;
    let mut buf = [0u8; BUF_SIZE];

    unsafe {
        let fd = open(name.as_ptr() as *const c_char, flags, mode);
        if fd == -1 {
            return eprintln!("Failed to open output file");
        }
        loop {
            let count = read(0 as c_int, buf.as_mut_ptr() as *mut c_void, BUF_SIZE);
            if count == -1 {
                break eprintln!("Failed to read stdin");
            }
            if count == 0 {
                break;
            }
            if write(1 as c_int, buf.as_ptr() as *const c_void, count as usize) == -1 {
                break eprintln!("Failed to write stdout");
            }
            if write(fd, buf.as_ptr() as *const c_void, count as usize) == -1 {
                break eprintln!("Failed to write output file");
            }
        }
        if close(fd) == -1 {
            return eprintln!("Failed to close output file");
        }
    }
}
