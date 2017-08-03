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
    let pty_resize = Pty::spawn(&get_shell(), &get_terminal_size().unwrap()).unwrap();

    let mut tty_output = get_tty().unwrap().into_raw_mode().unwrap();
    let mut pty_output = pty_resize.try_clone().unwrap();

    let mut tty_input = tty_output.try_clone().unwrap();
    let mut pty_input = pty_output.try_clone().unwrap();


    let handle1 = thread::spawn(move || {
        loop {
            let tty_file: &mut File = &mut tty_output;
            match pipe::<Pty, File>(&mut pty_input, tty_file) {
                Err(_) => return,
                _      => (),
            }
        }
    });

    let handle2 = thread::spawn(move || {
        loop {
            let tty_file: &mut File = &mut tty_input;
            match pipe::<File, Pty>(tty_file, &mut pty_output) {
                Err(_) => return,
                _      => (),
            }
        }
    });

    let handle3 = thread::spawn(move || {
        loop {
            signal.recv().unwrap();
            pty_resize.resize(&get_terminal_size().unwrap());
        }
    });

    handle1.join();
}

/// Sends the content of input into output
fn pipe<R: Read, W: Write>(input: &mut Read, output: &mut Write) -> Result<()> {
    let mut packet = [0; 4096];

    let count = input.read(&mut packet)?;

    let read = &packet[..count];
    output.write_all(&read)?;
    output.flush()?;

    Ok(())
}
