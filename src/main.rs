//const WALL :char = '#';
//const SPACE:char = ' ';

extern crate termion;

use termion::raw::IntoRawMode;
use termion::event::Key;
use std::io::{Write, stdout};
use crate::termion::input::TermRead;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Vec2 {
    fn limit_x(&mut self, n: f64) {
        if self.x > n {
            self.x = n;
        } else if self.x < -n {
            self.x = -n;
        }
    }

    fn limit_y(&mut self, n: f64) {
        if self.y > n {
            self.y = n;
        } else if self.y < -n {
            self.y = -n;
        }
    }
}

struct Player {
    pos:   Vec2,
    vel:   Vec2,
    accel: Vec2,
    ch:    char,
}

impl Player {
    fn update(&mut self, grid: &Vec<Vec<char>>) {
        self.vel += Vec2{x:0.0, y:0.1};
        self.vel.x *= 0.8;
        self.vel.limit_x(0.5);
        self.vel.limit_y(1.0);
        self.collide(&grid);
        self.pos += self.vel;
    }

    fn draw<W: Write>(&self, term: &mut Term<W>) {
        term.write_to(&self.ch.to_string(), self.pos.x as u16, self.pos.y as u16);
    }

    fn collide(&mut self, grid: &Vec<Vec<char>>) {
        let next_pos = self.pos + self.vel;
        let next_ch_x = grid[self.pos.y as usize][next_pos.x as usize];
        let next_ch_y = grid[next_pos.y as usize][self.pos.x as usize];
        let next_ch_xy = grid[next_pos.y as usize][next_pos.x as usize];
        let vel_x = next_pos.x as i64 - self.pos.x as i64;
        let vel_y = next_pos.y as i64 - self.pos.y as i64;
        
        if next_ch_y != ' ' {
            self.vel.y = 0.0;
        }

        if next_ch_x != ' ' {
            self.vel.x = 0.0;
        }
        
    }
}

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

	fn write_to(&mut self, text:&str, x:u16, y:u16) {
		write!(self.screen, "{}{}", termion::cursor::Goto(x+1, y+1), text).unwrap();
	}

}

fn draw_screen<W: Write>(screen: &mut Term<W>, file: &Vec<Vec<char>>) {
    let mut i = 0;
    for line in file {
        screen.goto(0, i);
        let str: String = line.into_iter().collect();
        screen.write(&str);
        i += 1;
    }
}

fn process_input(player: &mut Player, stdin: &mut termion::input::Keys<termion::AsyncReader>) -> bool {
    if let Some(c) = stdin.next() {
        match c.unwrap() {
            // Exit.
            Key::Char('q') => return true,
            Key::Ctrl('c') => return true,
            //Key::Char(c)   => println!("{}", c),
            //Key::Alt(c)    => println!("Alt-{}", c),
            Key::Left      => player.vel.x -= 0.2,
            Key::Right     => player.vel.x += 0.2,
            Key::Up        => player.vel.y -= 1.0,
            Key::Down      => println!("<down>"),
            _              => println!("Other"),
        }
    }
    return false;
}

fn main() {
    let file = std::fs::read_to_string("level.txt").expect("Could not open level file!");
    let split = file.lines();
    let mut grid: Vec<Vec<char>> = vec![];
    for line in split {
        grid.push(line.chars().collect());
    }
    
    let stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
	let screen = termion::screen::AlternateScreen::from(stdout).into_raw_mode().unwrap();
	let mut term = Term{screen};
	term.write_to(&termion::cursor::Hide.to_string(), 0, 0);

	let mut player = Player {
	    pos: Vec2{x: 1.0, y: 16.0},
        vel: Vec2{x: 0.0, y: 0.0},
        accel: Vec2{x: 0.0, y: 0.0},
        ch: '@',
	};

    loop {
	    draw_screen(&mut term, &grid);
	    if process_input(&mut player, &mut stdin) {
	        break;
	    }
	  	player.update(&grid);
	  	player.draw(&mut term);
	  	term.flush();
	    std::thread::sleep(std::time::Duration::from_millis(1000 / 30));
	}
	term.write_to(&termion::cursor::Show.to_string(), 0, 0);
    
    // Here the destructor is automatically called, and the terminal state is restored.
}
