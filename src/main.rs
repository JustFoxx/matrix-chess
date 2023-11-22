use std::fmt::{Display, Error, format, Formatter};
use std::io::{stdout, Write};
use colored::{Color, Colorize};

#[derive(PartialEq, Eq, Clone)]
enum Player {
    Blue = -1,
    Red = 1 ,
}

impl Player {
    fn get_color(&self) -> Color {
        match *self {
            Player::Blue => Color::Blue,
            Player::Red => Color::Red,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Piece {
    Empty,
    Pawn(Player),
    Knight(Player),
    Bishop(Player),
    Rook(Player),
    Queen(Player),
    King(Player),
    DoublePawn(Player),
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Empty => write!(f, " "),
            Piece::Pawn(player) | Piece::DoublePawn(player) => write!(f, "{}","P".color(player.get_color())),
            Piece::Knight(player) => write!(f, "{}", "N".color(player.get_color())),
            Piece::Bishop(player) => write!(f, "{}", "B".color(player.get_color())),
            Piece::Rook(player) => write!(f, "{}", "R".color(player.get_color())),
            Piece::Queen(player) => write!(f, "{}", "Q".color(player.get_color())),
            Piece::King(player) => write!(f, "{}", "K".color(player.get_color())),
        }
    }
}

type Board = [[Piece;8];8];

#[derive(PartialEq, Eq)]
struct Place {
    x: usize,
    y: usize,
}

impl Place {
    fn new(x: usize, y: usize) -> Place {
        Place {
            x,
            y
        }
    }
}

trait BoardTrait {
    fn print_board(&self) -> Result<(), std::io::Error>;
    fn move_piece_and_print(&mut self, place: Place, moved: Place) -> Result<(), &str>;
}

impl BoardTrait for Board {
    fn print_board(&self) -> Result<(), std::io::Error> {
        let mut lock = stdout().lock();
        for (i, row) in self.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let colored_cell = match (i + j) % 2 == 0 {
                    true => cell.to_string().on_white(),
                    false => cell.to_string().on_black(),
                };
                lock.write_all(colored_cell.to_string().as_bytes())?;
            }
            lock.write_all("\n".as_bytes())?;
            lock.flush()?;
        }
        Ok(())
    }

    fn move_piece_and_print(&mut self, from: Place, to: Place) -> Result<(), &str> {
        if to == from && to.x >= self.len() && to.y >= self.len() && from.x >= self.len() && from.y >= self.len() {
            return Err("Invalid move")
        }
        let coords = (
            (self.len()-1-from.y, from.x),
            (self.len()-1-to.y, to.x)
        );
        let piece = self[coords.0.0][coords.0.1].clone();
        match piece {
            Piece::Empty => return Err("No piece to move"),
            _ => {
                self[coords.1.0][coords.1.1] = piece;
                self[coords.0.0][coords.0.1] = Piece::Empty;
            }
        }
        match self.print_board() {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to print board"),
        }
    }

}

const CHESS_BOARD: Board = [
    [Piece::Rook(Player::Blue), Piece::Knight(Player::Blue), Piece::Bishop(Player::Blue), Piece::Queen(Player::Blue), Piece::King(Player::Blue), Piece::Bishop(Player::Blue), Piece::Knight(Player::Blue), Piece::Rook(Player::Blue)],
    [Piece::Pawn(Player::Blue), Piece::Pawn(Player::Blue), Piece::Pawn(Player::Blue), Piece::Pawn(Player::Blue), Piece::Pawn(Player::Blue), Piece::Pawn(Player::Blue), Piece::Pawn(Player::Blue), Piece::Pawn(Player::Blue)],
    [Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty],
    [Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty],
    [Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty],
    [Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty],
    [Piece::Pawn(Player::Red), Piece::Pawn(Player::Red), Piece::Pawn(Player::Red), Piece::Pawn(Player::Red), Piece::Pawn(Player::Red), Piece::Pawn(Player::Red), Piece::Pawn(Player::Red), Piece::Pawn(Player::Red)],
    [Piece::Rook(Player::Red), Piece::Knight(Player::Red), Piece::Bishop(Player::Red), Piece::Queen(Player::Red), Piece::King(Player::Red), Piece::Bishop(Player::Red), Piece::Knight(Player::Red), Piece::Rook(Player::Red)],
];

fn main() {
    let mut chess_board = CHESS_BOARD;
    chess_board.print_board().unwrap();
    chess_board.move_piece_and_print(Place::new(0,0), Place::new(0,2)).unwrap();
    chess_board.move_piece_and_print(Place::new(0,2), Place::new(1,4)).unwrap();
}
