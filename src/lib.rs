extern crate libc;

use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{AsRawFd, IntoRawFd, FromRawFd};
use std::ffi::{CStr, CString};
use std::io::{self, Write, Read};
use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;
use std::ops;

pub struct Pty {
    master: File
}

#[derive(Debug)]
pub enum PtyError {
    OpenPty,
    SpawnShell,
}

pub struct WinSize {
    pub width:  u16,
    pub height: u16,
}

impl WinSize {
    fn to_c_winsize(&self) -> libc::winsize {
        libc::winsize {
            ws_row:    self.width,
            ws_col:    self.height,
            // Unused
            ws_xpixel: 0,
            // Unused
            ws_ypixel: 0,
        }
    }
}

type RawFd = libc::c_int;

impl Pty {
    pub fn spawn(shell: &str, size: &WinSize) -> Result<Pty, PtyError> {
        let (master_fd, tty_path) = getpty(&size)?;
        let (stdin, stdout, stderr) = slave_stdio(&tty_path)?;
        
        let result = Command::new(&shell)
            .stdin(unsafe  { Stdio::from_raw_fd(stdin.as_raw_fd()) })
            .stdout(unsafe { Stdio::from_raw_fd(stdout.as_raw_fd()) })
            .stderr(unsafe { Stdio::from_raw_fd(stderr.as_raw_fd()) })
            .before_exec(before_exec)
            .spawn();

        match result {
            Ok(mut process) => {
                drop(stdin);
                drop(stdout);
                drop(stderr);
                Ok(Pty {
                    master: unsafe { File::from_raw_fd(master_fd) }
                })
            },
            Err(_) => Err(PtyError::SpawnShell)
        }
    }
}

impl Read for Pty {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.master.read(buf)
    }
}

impl Write for Pty {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.master.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.master.flush()
    }
}

impl ops::Deref for Pty {
    type Target = File;

    fn deref(&self) -> &File {
        &self.master
    }
}

impl ops::DerefMut for Pty {
    fn deref_mut(&mut self) -> &mut File {
        &mut self.master
    }
}

trait FromLibcResult: Sized {
    fn to_result(self) -> Result<Self, ()>;
}

impl FromLibcResult for libc::c_int {
    fn to_result(self) -> Result<Self, ()> {
        match self {
            -1  => Err(()),
            res => Ok(res),
        }
    }
}

fn getpty(size: &WinSize) -> Result<(RawFd, String), PtyError> {
    let master_fd = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_NONBLOCK)
            .open("/dev/ptmx")
            .map_err(|_| PtyError::OpenPty)?
            .into_raw_fd();

    unsafe {
        libc::grantpt(master_fd)
            .to_result()
            .map_err(|_| PtyError::OpenPty)?;

        libc::unlockpt(master_fd)
            .to_result()
            .map_err(|_| PtyError::OpenPty)?;

        libc::ioctl(master_fd, libc::TIOCSWINSZ, &size.to_c_winsize())
            .to_result()
            .map_err(|_| PtyError::OpenPty)?;
    }

    let tty_path = unsafe { CStr::from_ptr(libc::ptsname(master_fd)).to_string_lossy().into_owned() };
    Ok((master_fd, tty_path))
}

fn slave_stdio(tty_path: &str) -> Result<(File, File, File), PtyError> {
    let tty_c = CString::new(tty_path).unwrap();

    let stdin = unsafe { File::from_raw_fd(
        libc::open(tty_c.as_ptr(), libc::O_CLOEXEC | libc::O_RDONLY)
            .to_result()
            .map_err(|_| PtyError::OpenPty)?
    ) };
    let stdout = unsafe { File::from_raw_fd(
        libc::open(tty_c.as_ptr(), libc::O_CLOEXEC | libc::O_WRONLY)
            .to_result()
            .map_err(|_| PtyError::OpenPty)?
    ) };
    let stderr = unsafe { File::from_raw_fd(
        libc::open(tty_c.as_ptr(), libc::O_CLOEXEC | libc::O_WRONLY)
            .to_result()
            .map_err(|_| PtyError::OpenPty)?
    ) };

    Ok((stdin, stdout, stderr))
}

fn before_exec() -> io::Result<()> {
    unsafe {
        libc::setsid()
            .to_result()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, ""))?;

        libc::ioctl(0, libc::TIOCSCTTY, 1)
            .to_result()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, ""))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Write, Read};

    #[test]
    fn can_open_a_shell_with_its_own_pty_and_can_read_and_write_to_its_master_side() {
        // Opening shell and its pty
        let mut pty = Pty::spawn("/bin/sh", &WinSize { width: 100, height: 100 }).unwrap();

        let mut packet = [0; 4096];
        let mut count_read = 0;

        // Reading
        {
            loop {
                match pty.read(&mut packet) {
                    Err(_)    => continue,
                    Ok(0)     => continue,
                    Ok(count) => { count_read = count; break; }
                }
            }

            assert_eq!(2, count_read);

            let output = String::from_utf8_lossy(&packet[..count_read]);
            assert_eq!("$ ", &output);
        }

        // Writing and reading effect
        {
            pty.write_all("exit\n".as_bytes()).unwrap();
            pty.flush().unwrap();

            loop {
                match pty.read(&mut packet) {
                    Err(_)    => continue,
                    Ok(0)     => continue,
                    Ok(count) => { count_read = count; break; }
                }
            }

            assert_eq!(6, count_read);

            let output = String::from_utf8_lossy(&packet[..count_read]);
            assert_eq!("exit\r\n", &output);
        }
    }
}
