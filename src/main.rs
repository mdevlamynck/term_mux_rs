extern crate term_mux;
extern crate termion;

use termion::get_tty;
use termion::raw::IntoRawMode;
use term_mux::pty::Pty;
use term_mux::tui::Size;
use std::io::{Read, Write, Result};
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main () {
    let mut tty_output = get_tty().unwrap().into_raw_mode().unwrap();
    let mut pty_output = Pty::spawn("/bin/sh", &Size{ width: 100, height: 100}).unwrap();

    let mut tty_input = tty_output.try_clone().unwrap();
    let mut pty_input = pty_output.try_clone().unwrap();

    let handle1 = thread::spawn(move || {
        loop {
            let tty_file: &mut File = &mut tty_output;
            match pipe_input_to_output::<Pty, File>(&mut pty_input, tty_file) {
                Err(_) => return,
                _      => (),
            }
        }
    });

    let handle2 = thread::spawn(move || {
        loop {
            let tty_file: &mut File = &mut tty_input;
            match pipe_input_to_output::<File, Pty>(tty_file, &mut pty_output) {
                Err(_) => return,
                _      => (),
            }
        }
    });

    handle1.join();
}

fn pipe_input_to_output<R: Read, W: Write>(input: &mut Read, output: &mut Write) -> Result<()> {
    let mut packet = [0; 4096];

    let count = input.read(&mut packet)?;

    let read = &packet[..count];
    output.write_all(&read)?;
    output.flush()?;

    Ok(())
}
