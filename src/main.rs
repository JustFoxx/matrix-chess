use owo_colors::{AnsiColors, OwoColorize};
use std::fmt::{Display, Formatter, Write};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Player {
    Blue,
    Red,
}

impl Player {
    fn get_color(&self) -> AnsiColors {
        match *self {
            Player::Blue => AnsiColors::Blue,
            Player::Red => AnsiColors::Red,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Piece {
    Empty,
    Pawn(Player),
    Knight(Player),
    Bishop(Player),
    Rook(Player),
    Queen(Player),
    King(Player),
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Empty => write!(f, " "),
            Piece::Pawn(player) => {
                write!(f, "{}", "P".color(player.get_color()))
            }
            Piece::Knight(player) => write!(f, "{}", "N".color(player.get_color())),
            Piece::Bishop(player) => write!(f, "{}", "B".color(player.get_color())),
            Piece::Rook(player) => write!(f, "{}", "R".color(player.get_color())),
            Piece::Queen(player) => write!(f, "{}", "Q".color(player.get_color())),
            Piece::King(player) => write!(f, "{}", "K".color(player.get_color())),
        }
    }
}

struct Board {
    board: [[Piece; 8]; 8],
}

#[derive(PartialEq, Eq)]
struct Place {
    x: u8,
    y: u8,
}

impl Place {
    pub fn new(x: u8, y: u8) -> Place {
        Place { x, y }
    }
    pub fn y(&self) -> usize {
        self.y as usize
    }
    pub fn x(&self) -> usize {
        self.x as usize
    }
}

impl From<(u8, u8)> for Place {
    fn from((x, y): (u8, u8)) -> Self {
        Place::new(x, y)
    }
}
impl From<[u8; 2]> for Place {
    fn from([x, y]: [u8; 2]) -> Self {
        Place::new(x, y)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.board.iter().enumerate() {
            if i != 0 {
                f.write_char('\n')?;
            }
            for (j, piece) in row.iter().enumerate() {
                let color = match (i + j) % 2 == 0 {
                    true => AnsiColors::White,
                    false => AnsiColors::Black,
                };
                piece.on_color(color).fmt(f)?;
            }
        }
        Ok(())
    }
}

impl Board {
    fn move_piece(&mut self, from: impl Into<Place>, to: impl Into<Place>) -> Result<(), &str> {
        const SIZE: usize = 8;
        let (from, to) = (from.into(), to.into());
        if to == from || to.x() >= SIZE || to.y() >= SIZE || from.x() >= SIZE || from.y() >= SIZE {
            return Err("Invalid move");
        }
        let [from, to] = [(SIZE - 1 - from.y(), from.x()), (SIZE - 1 - to.y(), to.x())];
        let piece = self.board[from.0][from.1];
        match piece {
            Piece::Empty => return Err("No piece to move"),
            _ => {
                self.board[to.0][to.1] = piece;
                self.board[from.0][from.1] = Piece::Empty;
            }
        }
        Ok(())
    }
}

const CHESS_BOARD: Board = {
    use Piece::*;
    use Player::*;
    let board = [
        [
            Rook(Blue),
            Knight(Blue),
            Bishop(Blue),
            Queen(Blue),
            King(Blue),
            Bishop(Blue),
            Knight(Blue),
            Rook(Blue),
        ],
        [Pawn(Blue); 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Pawn(Red); 8],
        [
            Rook(Red),
            Knight(Red),
            Bishop(Red),
            Queen(Red),
            King(Red),
            Bishop(Red),
            Knight(Red),
            Rook(Red),
        ],
    ];
    Board { board }
};

fn main() {
    let mut chess_board = CHESS_BOARD;
    println!("{chess_board}");
    chess_board.move_piece((0, 0), (0, 2)).unwrap();
    println!("{chess_board}");
    chess_board.move_piece((0, 2), (1, 4)).unwrap();
    println!("{chess_board}");
}
