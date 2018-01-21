# term_mux_rs (working title, I don't like it, see [#1](https://github.com/mdevlamynck/term_mux_rs/issues/1))

[![Build Status](https://travis-ci.org/mdevlamynck/term_mux_rs.svg?branch=master)](https://travis-ci.org/mdevlamynck/term_mux_rs)

Terminal multiplexer in rust.

# About

Term_mux_rs is mainly intented to be used along [`alacritty`](https://github.com/jwilm/alacritty) and will try to be fast enough to keep up with it.

The software is in a early development state and not usable yet.

This project only supports GNU/Linux at the moment. Redox, macOS and Windows support may happen in the future (in that order of priority).

# Installation

You can use `cargo install` to install this project. It will compile the binary `term_mux` and install it in the `~/.cargo/bin` folder. Make sure this folder is in your path if you want to be able to run it directly.

```sh
cargo install --git https://github.com/mdevlamynck/term_mux_rs

# or you can specify a branch with --branch
cargo install --git https://github.com/mdevlamynck/term_mux_rs --branch dev
```

If you want the last stable version (i.e. release), use the master branch.
If you want the last development version, use the dev branch.

This project is not ready to be used yet. Once the project is ready, it will be published on crates.io and you will be able to install the latest release with a simple `cargo install term_mux_rs`.

# Contributing

Thank you for your interest in working on this project! To get you started you can read the [Hacking](#hacking) section.

Contributions are welcome! Please submit issues if you find bugs.

Proper documentation on the project and how it's structured will be added once the project starts to grow a bit and things start to take form. In the meantime don't hesitate to reach out and ask questions through github or good old email!

# Hacking

As any rust project, use `cargo` to build, run the project, run the tests or build the docs.

```sh
cargo build      # compile
cargo run        # launch term_mux
cargo test       # run tests
cargo doc        # build the docs
cargo doc --open # build the docs and open them in your browser
```

As a bonus, if you want to see the full documentation, including the docs of private elements, use :

```sh
cargo rustdoc -- --no-defaults --passes collapse-docs --passes unindent-comments --passes strip-priv-imports

# or the version with --open
cargo rustdoc --open -- --no-defaults --passes collapse-docs --passes unindent-comments --passes strip-priv-imports
```

This doc also includes the documentation of libraries term_mux_rs depends on so it can be really usefull when working on the project.

As for the documentation of rust itself, if you're using `rustup` you can use `rustup doc`.
