extern crate term_mux;
extern crate termion;

use termion::get_tty;
use termion::raw::IntoRawMode;
use term_mux::pty::Pty;
use term_mux::tui::Size;
use std::io::{Read, Write};
use std::fs::File;

fn main () {
    let mut tty = get_tty().unwrap().into_raw_mode().unwrap();
    let mut pty = Pty::spawn("/bin/sh", &Size{ width: 100, height: 100}).unwrap();

    let tty_file: &mut File = &mut tty;

    loop {
        pipe_input_to_output::<Pty, File>(&mut pty, tty_file);
        pipe_input_to_output::<File, Pty>(tty_file, &mut pty);
    }
}

fn pipe_input_to_output<R: Read, W: Write>(input: &mut Read, output: &mut Write) {
    let mut packet = [0; 4096];
    let read_count: usize;

    loop {
        match input.read(&mut packet) {
            Err(_)    => continue,
            Ok(0)     => continue,
            Ok(count) => { read_count = count ; break }
        };
    }

    let read = &packet[..read_count];
    output.write_all(&read).unwrap();
    output.flush().unwrap();
}
