extern crate term_mux;
extern crate termion;
extern crate chan_signal;

use std::io::{self, Read, Write};
use std::fs::File;
use std::thread;
use termion::get_tty;
use termion::raw::IntoRawMode;
use chan_signal::{notify, Signal};
use term_mux::pty::Pty;
use term_mux::tui::get_terminal_size;
use term_mux::util::get_shell;

fn main () {
    let signal         = notify(&[Signal::WINCH]);

    let mut tty_output = get_tty().unwrap().into_raw_mode().unwrap();
    let mut tty_input  = tty_output.try_clone().unwrap();

    let pty_resize     = Pty::spawn(&get_shell(), &get_terminal_size().unwrap()).unwrap();
    let mut pty_output = pty_resize.try_clone().unwrap();
    let mut pty_input  = pty_output.try_clone().unwrap();

    let handle = thread::spawn(move || pipe(&mut pty_input, &mut tty_output));
    let _      = thread::spawn(move || pipe(&mut tty_input, &mut pty_output));

    thread::spawn(move || -> Result<(), ()> {
        for _ in signal.iter() {
            pty_resize.resize(&get_terminal_size()?).map_err(|_| ())?;
        }

        Ok(())
    });

    let _ = handle.join();
}

/// Sends the content of input into output
fn pipe(input: &mut File, output: &mut File) -> io::Result<()> {
    let mut buf = [0; 1];

    loop {
        let count = input.read(&mut buf)?;
        output.write_all(&buf[..count])?;
    }
}
