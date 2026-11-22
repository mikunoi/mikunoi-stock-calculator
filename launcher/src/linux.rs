use std::fs::{self, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::process::Command;

extern "C" {
    fn flock(fd: i32, operation: i32) -> i32;
}

fn run() -> io::Result<i32> {
    let mut args = std::env::args_os().skip(1);
    let python = args.next().ok_or_else(|| io::Error::other("Missing Python path"))?;
    let script = args.next().ok_or_else(|| io::Error::other("Missing UI path"))?;
    let data = std::env::var_os("XDG_DATA_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|v| PathBuf::from(v).join(".local/share")))
        .ok_or_else(|| io::Error::other("Missing user data directory"))?
        .join("IndependentInvestmentCalculator");
    fs::create_dir_all(&data)?;
    let lock = OpenOptions::new().write(true).create(true).mode(0o600)
        .open(data.join("calculator.lock"))?;
    const LOCK_EX_NB: i32 = 2 | 4;
    if unsafe { flock(lock.as_raw_fd(), LOCK_EX_NB) } != 0 {
        return Err(io::Error::last_os_error());
    }
    let status = Command::new(python).arg(script).args(args).status()?;
    Ok(status.code().unwrap_or(1))
}

fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            eprintln!("Calculator could not start: {error}");
            std::process::exit(1);
        }
    }
}
