extern crate term_mux;
extern crate termion;
extern crate chan_signal;

use std::io::{Read, Write, Result};
use std::fs::File;
use std::thread;
use std::time::Duration;
use termion::get_tty;
use termion::raw::IntoRawMode;
use chan_signal::{notify, Signal};
use term_mux::pty::Pty;
use term_mux::tui::{get_terminal_size, Size};
use term_mux::get_shell;

fn main () {
    let signal = notify(&[Signal::WINCH]);

    let (pty, mut pty_input) = Pty::spawn(&get_shell(), &get_terminal_size().unwrap()).unwrap();

    let mut tty_input = get_tty().unwrap().into_raw_mode().unwrap();
    let mut tty_output = tty_input.try_clone().unwrap();
    let mut pty_output = pty_input.try_clone().unwrap();

    let handle = thread::spawn(move || {
        loop {
            match pipe(&mut pty_input, &mut tty_output) {
                Err(_) => return,
                _      => (),
            }
        }
    });

    thread::spawn(move || {
        loop {
            match pipe(&mut tty_input, &mut pty_output) {
                Err(_) => return,
                _      => (),
            }
        }
    });

    thread::spawn(move || {
        loop {
            signal.recv().unwrap();
            pty.resize(&get_terminal_size().unwrap());
        }
    });

    handle.join();
}

/// Sends the content of input into output
fn pipe(input: &mut File, output: &mut File) -> Result<()> {
    let mut packet = [0; 4096];

    let count = input.read(&mut packet)?;

    let read = &packet[..count];
    output.write_all(&read)?;
    output.flush()?;

    Ok(())
}
