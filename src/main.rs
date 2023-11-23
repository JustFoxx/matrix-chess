use std::fmt::{Display, Formatter, Write};
use owo_colors::{AnsiColors, FgDynColorDisplay, OwoColorize};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Player {
    A,
    B,
}

impl Player {
    fn get_color(&self) -> AnsiColors {
        match *self {
            Player::A => AnsiColors::Blue,
            Player::B => AnsiColors::Red,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Piece {
    Empty,
    Pawn(Player, bool),
    Knight(Player),
    Bishop(Player),
    Rook(Player),
    Queen(Player),
    King(Player),
}

impl Piece {
    fn get_color(&self) -> AnsiColors {
        match self {
            Piece::Empty => AnsiColors::Black,
            Piece::Pawn(player,_) |
            Piece::Knight(player) |
            Piece::Bishop(player) |
            Piece::Rook(player)   |
            Piece::Queen(player)  |
            Piece::King(player) => player.get_color(),
        }
    }

    fn get_colored_str(&self) -> FgDynColorDisplay<'_, AnsiColors, char> {
        self.get_str().color(self.get_color())
    }

    fn get_str(&self) -> &char {
        match self {
            Piece::Empty => &' ',
            Piece::Pawn(_,_) => &'P',
            Piece::Knight(_) => &'N',
            Piece::Bishop(_) => &'B',
            Piece::Rook(_) => &'R',
            Piece::Queen(_) => &'Q',
            Piece::King(_) => &'K',
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let piece = self.get_colored_str();
        piece.fmt(f)
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
            Rook(A),
            Knight(A),
            Bishop(A),
            Queen(A),
            King(A),
            Bishop(A),
            Knight(A),
            Rook(A),
        ],
        [Pawn(A, false); 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Pawn(B, false); 8],
        [
            Rook(B),
            Knight(B),
            Bishop(B),
            Queen(B),
            King(B),
            Bishop(B),
            Knight(B),
            Rook(B),
        ],
    ];
    Board { board }
};

fn main() {
    let mut chess_board = CHESS_BOARD;
    println!("{chess_board}\n");
    chess_board.move_piece((0, 0), (0, 2)).unwrap();
    println!("{chess_board}\n");
    chess_board.move_piece((0, 2), (1, 4)).unwrap();
    println!("{chess_board}\n");
}
