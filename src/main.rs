//const WALL :char = '#';
//const SPACE:char = ' ';

extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

struct Term<W: Write> {
	screen: W,
}

impl<W: Write> Term<W> {
	
	fn goto(&mut self, x: u16, y: u16) {
		write!(self.screen, "{}", termion::cursor::Goto(x+1, y+1)).unwrap();
	}

	fn flush(&mut self) {
		self.screen.flush().unwrap();
	}

	fn write(&mut self, text:&str) {
		write!(self.screen, "{}", text).unwrap();
	}

	fn write(&mut self, text:&str, x:u16, y:u16) {
		write!(self.screen, "{}{}", termion::cursor::Goto(x+1, y+1), text).unwrap();
	}

}

fn draw_screen<W: Write>(screen: &mut Term<W>) {
	//write!(screen, "Hey there.").unwrap();
    //write!(screen, "{}", termion::cursor::Goto(1, 1)).unwrap();
    //write!(screen, "ee").unwrap();
    //screen.flush().unwrap();
    screen.goto(0, 0);
    screen.write("this is text");
    screen.flush();
}

fn main() {
    // Enter raw mode.
    let stdout = stdout().into_raw_mode().unwrap();
	let screen = termion::screen::AlternateScreen::from(stdout).into_raw_mode().unwrap();
	let mut term = Term {screen};
	draw_screen(&mut term);
    
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Here the destructor is automatically called, and the terminal state is restored.
}
