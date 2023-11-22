use std::fmt::{Display, Formatter};
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

enum Move {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
    UpLeft(u8),
    UpRight(u8),
    DownLeft(u8),
    DownRight(u8),
}

type Board = [[Piece;8];8];

trait BoardTrait {
    fn print_board(&self);
    fn move_piece_and_print(&mut self, place: (usize, usize), moved: Move) -> Result<(), &str>;
}

impl BoardTrait for Board {
    fn print_board(&self) {
        for (i, row) in self.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let colored_cell = match (i + j) % 2 == 0 {
                    true => cell.to_string().on_white(),
                    false => cell.to_string().on_black(),
                };
                print!("{}", colored_cell);
            }
            println!();
        }
    }

    fn move_piece_and_print(&mut self, place: (usize, usize), moved: Move) -> Result<(), &str> {
        let piece = self[place.0][place.1].clone();
        match piece {
            Piece::Empty => return Err("No piece to move"),
            _ => {
                match moved {
                    Move::Up(i) => self[place.0 + i as usize][place.1] = piece,
                    Move::Down(i) => self[place.0 - i as usize][place.1] = piece,
                    Move::Left(i) => self[place.0][place.1 + i as usize] = piece,
                    Move::Right(i) => self[place.0][place.1 - i as usize] = piece,
                    Move::UpLeft(i) => self[place.0 + i as usize][place.1 - i as usize] = piece,
                    Move::UpRight(i) => self[place.0 + i as usize][place.1 + i as usize] = piece,
                    Move::DownLeft(i) => self[place.0 - i as usize][place.1 - i as usize] = piece,
                    Move::DownRight(i) => self[place.0 - i as usize][place.1 + i as usize] = piece,
                }
                self[place.0][place.1] = Piece::Empty;
            }
        }
        self.print_board();
        Ok(())
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
    chess_board.print_board();
    chess_board.move_piece_and_print((0,0), Move::Up(2)).unwrap();
}
