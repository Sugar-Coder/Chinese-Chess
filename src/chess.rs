use crate::{configs::{BH, BW, GL}, pos::Pos};
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
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

pub enum Action {
    Go(Pos),
    Take(Pos),
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
    pub center: Vec2, // world coordinate of the center of the board
}

impl Board {
    pub fn new() -> Board {
        Board { 
            points: vec![None; BW * BH],
            center: Vec2::new(0.0, 0.0),
        }
    }

    pub fn i(&self, pos: Pos) -> usize {
        return pos.0 as usize + pos.1 as usize * BW;
    }

    pub fn pos(&self, i: usize) -> Pos {
        return Pos{0: (i % BW) as i32, 1: (i / BW) as i32};
    }

    fn in_bound(&self, pos: Pos) -> bool {
        return pos.0 <= 8 && pos.0 >= 0 && pos.1 >= 0 && pos.1 <= 9;
    }

    pub fn get(&self, pos: Pos) -> Option<&Grid> {
        if !self.in_bound(pos) {
            return None;
        }
        return Some(&self.points[self.i(pos)]);
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

    pub fn in_bound(&self, world_position: &Vec2) -> bool {
        let on_board_position = *world_position - self.board.center;
        if on_board_position.x >= -4.5 * GL && on_board_position.x <= 4.5 * GL &&
        on_board_position.y >= -5. * GL && on_board_position.y <= 5. * GL {
                return true;
        } else {
            return false;
        }
    }

    pub fn world_to_board(&self, world_position: &Vec2) -> Pos {
        // world position must on board
        let on_board_position = *world_position - self.board.center;
        return Pos {
            0: ((on_board_position.x + 4.5 * GL) / GL) as i32,
            1: ((on_board_position.y + 5.0 * GL) / GL) as i32,
        }
    }

    pub fn board_to_world(&self, pos: Pos) -> Transform {
        Transform::from_xyz(
            (pos.0 as f32 - 4.0) * GL + self.board.center.x, 
            (pos.1 as f32 - 4.5) * GL + self.board.center.y, 
            1.0
        )
    }

    // for movement judgement
    pub fn save_moves(&self, piece: Piece, from: Pos) -> Vec<Action> {
        self.board.filter_save_moves(
            self.player.unwrap(),
            from,
            piece.moves(&self.board, from, self.player.unwrap()),
        )
    }

    pub fn playable_moves(&self, from: Pos) -> Option<Vec<Action>> {
        if let Some(Some((color, piece))) = self.board.get(from) {
            if self.turn == 0 { // who first attemp to move
                self.player = Some(*color);
            }
            if self.player == Some(*color) {
                return Some(self.save_moves(*piece, from));
            }
        }
        None
    }

    pub fn playable_move(&self, from: Pos, to: Pos) -> Option<Action> {
        if let Some(moves) = self.playable_moves(from) {
            for action in moves {
                match action {
                    Action::Go(pos) => {
                        if pos == to {
                            return Some(action);
                        }
                    },
                    Action::Take(pos) => {
                        if pos == to {
                            return Some(action);
                        }
                    },
                }
            }
        }
        None
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