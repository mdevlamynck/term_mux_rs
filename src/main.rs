extern crate term_mux;
extern crate termion;

use termion::get_tty;
use termion::raw::IntoRawMode;
use term_mux::pty::{Pty, WinSize};
use std::io::{Read, Write};
use std::fs::File;

fn main () {
    let mut tty = get_tty().unwrap().into_raw_mode().unwrap();
    let mut pty = Pty::spawn("/bin/sh", &WinSize{ width: 100, height: 100}).unwrap();

    loop {
        pipe_input_to_output(&mut pty, &mut tty);
        pipe_input_to_output(&mut tty, &mut pty);
    }
}

fn pipe_input_to_output(input: &mut File, output: &mut File) {
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
