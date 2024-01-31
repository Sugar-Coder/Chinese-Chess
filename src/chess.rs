use crate::{configs::{BH, BW}, pos::Pos};
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum PlayerColor {
    Black,
    Red,
}

impl PlayerColor {
    pub fn next(self) -> Self {
        match self {
            PlayerColor::Black => PlayerColor::Red,
            PlayerColor::Red => PlayerColor::Black,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Piece {
    Jiang,
    Shi,
    Xiang,
    Ma,
    Che,
    Pao,
    Bing,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Jiang => write!(f, "jiang"),
            Piece::Shi => write!(f, "shi"),
            Piece::Xiang => write!(f, "xiang"),
            Piece::Ma => write!(f, "ma"),
            Piece::Che => write!(f, "che"),
            Piece::Pao => write!(f, "pao"),
            Piece::Bing => write!(f, "bing"),
        }
    }
}

type Grid = Option<(PlayerColor, Piece)>;

pub struct Board {
    pub points: Vec<Grid>,
}

impl Board {
    pub fn new() -> Board {
        Board { points: vec![None; BW * BH] }
    }

    pub fn i(&self, pos: Pos) -> usize {
        return pos.0 as usize + pos.1 as usize * BW;
    }

    pub fn pos(&self, i: usize) -> Pos {
        return Pos{0: (i % BW) as i32, 1: (i / BW) as i32};
    }
}

#[derive(Resource)]
pub struct ChessGame {
    pub board: Board,
    pub turn: u32,
    pub player: Option<PlayerColor>,
}

impl ChessGame {
    pub fn new() -> Self {
        ChessGame {
            board: make_board(),
            turn: 0,
            player: None,
        }
    }
}

fn make_board() -> Board {
    let mut board = Board::new();
    let pieces = vec![Piece::Che, Piece::Ma, Piece::Xiang, Piece::Shi, Piece::Jiang, Piece::Shi, Piece::Xiang, Piece::Ma, Piece::Che];
    for i in 0..=8 {
        board.points[i] = Some((PlayerColor::Red, pieces[i]));
        board.points[i + 9 * BW] = Some((PlayerColor::Black, pieces[i]));
    }
    for i in vec![1, 7] {
        let mut idx = board.i(Pos{0: i, 1: 7});
        board.points[idx] = Some((PlayerColor::Black, Piece::Pao));
        idx = board.i(Pos{0: i, 1: 2});
        board.points[idx] = Some((PlayerColor::Red, Piece::Pao));
    }
    for i in vec![0, 2, 4, 6, 8] {
        let mut idx = board.i(Pos{0: i, 1: 6});
        board.points[idx] = Some((PlayerColor::Black, Piece::Bing));
        idx = board.i(Pos{0: i, 1: 3});
        board.points[idx] = Some((PlayerColor::Red, Piece::Bing));
    }
    board
}