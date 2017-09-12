//! ANSI Terminal Stream Parsing

use vte::{Perform, Parser};

/// Actions one can perform on a terminal through ansi escapes.
#[derive(Mock)]
pub trait TermAction {
	/// Prints a character and move the cursor.
	fn print(&mut self, c: char) {}

	/// Moves the cursor.
	fn move_cursor(&mut self, movement: Movement) {}

	/// Clears a part of the terminal
	fn clear(&mut self, clear: Clear) {}
}

/// Cursor movement
#[derive(Debug)]
pub struct Movement (Col, Row);

/// Column movement
#[derive(Debug)]
pub enum Col {
	/// Absolute movement
	Abs(u16),
	/// Relative movement
	Rel(i16)
}

/// Row movement
#[derive(Debug)]
pub enum Row {
	/// Absolute movement
	Abs(u16),
	/// Relative movement
	Rel(i16)
}

/// Possible screen clear modes
#[derive(Debug)]
pub enum Clear {
	/// Current line from cursor to the left side
	Left,
	/// Current line from cursor to the right side
	Right,
	/// Entire current line
	Line,
	/// All lines above the cursor to the top
	Above,
	/// All lines bellow the cursor to the bottom
	Bellow,
	/// Entire screen
	All,
	/// History
	Scrollback
}

/// Parses a stream of ansi escapes and calls
fn parse<'a, I: Iterator<Item=&'a u8>, TA: TermAction>(input: I, terminal: &mut TA) {
    let mut parser    = Parser::new();
	let mut performer = Performer::new(terminal);

    for byte in input {
	    parser.advance(&mut performer, *byte);
    }
}

struct Performer<'a, TA: TermAction + 'a> {
    action: &'a mut TA,
}

impl<'a, TA: TermAction + 'a> Performer<'a, TA> {
    pub fn new(action: &'a mut TA) -> Self {
        Performer { action }
    }
}

impl<'a, TA: TermAction + 'a> Perform for Performer<'a, TA> {
	fn print(&mut self, c: char) {
		self.action.print(c);
	}

	fn execute(&mut self, byte: u8) {
		match byte {
			//c0::CR => Movement (Col::Rel(0), Row::Rel(0)),
			_ => unimplemented!(),
		};
	}

	fn osc_dispatch(&mut self, params: &[&[u8]]) {unimplemented!()}

	fn csi_dispatch(&mut self, params: &[i64], intermediates: &[u8], ignore: bool, _: char) {unimplemented!()}

	fn esc_dispatch(&mut self, params: &[i64], intermediates: &[u8], ignore: bool, byte: u8) {unimplemented!()}

    // Not handled at the moment
	fn hook(&mut self, params: &[i64], intermediates: &[u8], ignore: bool) {unimplemented!()}

    // Not handled at the moment
	fn put(&mut self, byte: u8) {unimplemented!()}

    // Not handled at the moment
	fn unhook(&mut self) {unimplemented!()}
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn empty_stream_does_nothing() {
		assert_mock_sequence!(
			TermAction,
			[],
			|mut mock| parse(b"".into_iter(), &mut mock)
	    );
	}

    #[test]
    fn prints_text() {
	    assert_mock_sequence!(
			TermAction,
			[
				print_call('t'),
				print_call('e'),
				print_call('s'),
				print_call('t')
			],
			|mut mock| parse(b"test".into_iter(), &mut mock)
	    );
    }

	#[test]
	fn converts_invalid_utf8_content() {
		assert_mock_sequence!(
			TermAction,
			[
				print_call('a'),
				print_call('�'),
				print_call('c')
			],
			|mut mock| parse(b"a\xF0\x00c".into_iter(), &mut mock)
	    );
	}

	#[test]
	fn move_the_cursor_around() {
		assert_mock_sequence!(
			TermAction,
			[
				//move_cursor_call(Movement (Col::Abs(1), Row::Abs(1))),
			],
			|mut mock| parse(b"".into_iter(), &mut mock)
	    );
	}
}
