use crate::{configs::{BH, BW, GL, BCX, BCY}, pos::Pos};
use crate::pieces::{Action, Piece, PlayerColor};
use bevy::prelude::*;


type Grid = Option<(PlayerColor, Piece)>;

#[derive(Clone)]
pub struct Board {
    pub points: Vec<Grid>,
    pub center: Vec2, // world coordinate of the center of the board
}

impl Board {
    pub fn new() -> Board {
        Board { 
            points: vec![None; BW * BH],
            center: Vec2::new(BCX, BCY),
        }
    }

    pub fn i(&self, pos: Pos) -> usize {
        return pos.0 as usize + pos.1 as usize * BW;
    }

    pub fn pos(&self, i: usize) -> Pos {
        return Pos{0: (i % BW) as i32, 1: (i / BW) as i32};
    }

    fn in_board(&self, pos: Pos) -> bool {
        return pos.0 <= 8 && pos.0 >= 0 && pos.1 >= 0 && pos.1 <= 9;
    }

    pub fn get(&self, pos: Pos) -> Option<&Grid> {
        // 超出界外返回None
        if !self.in_board(pos) {
            return None;
        }
        return Some(&self.points[self.i(pos)]);
    }

    pub fn set(&mut self, pos: Pos, grid: Grid) {
        if self.in_board(pos) {
            let idx = self.i(pos);
            self.points[idx] = grid;
        }
    }

    pub fn filter_save_moves(&self, color: PlayerColor, from: Pos, actions: Vec<Action>) -> Vec<Action> {
        info!(
            "available actions count={}",
            actions.len()
        );
        actions
    }

    pub fn play(&self, from: Pos, action: Action) -> Self {
        let mut res = self.clone();
        if let Some((c, p)) = self.get(from).unwrap() {
            match action {
                Action::Go(to) => {
                    res.set(from, None);
                    info!("setting Pos{} to {}", to, *p);
                    res.set(to, Some((*c, *p)));
                },
                Action::Take(to) => {
                    info!("{} taking Pos{}", *p, to);
                    res.set(from, None);
                    res.set(to, Some((*c, *p)));
                }
            }
        }
        res
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
    pub fn save_moves(&self, piece: Piece, from: Pos, color: PlayerColor) -> Vec<Action> {
        self.board.filter_save_moves(
            color,
            from,
            piece.moves(&self.board, from, color),
        )
    }

    pub fn playable_moves(&self, from: Pos) -> Option<Vec<Action>> {
        if let Some(Some((color, piece))) = self.board.get(from) {
            // if self.turn == 0 { // who first attemp to move
            //     self.player = Some(*color);
            // }
            if self.turn == 0 || self.player == Some(*color) {
                return Some(self.save_moves(*piece, from, *color));
            } else {
                info!("Not your turn");
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

    pub fn play(&mut self, from: Pos, action: Action) {
        if let Some((c, _)) = self.board.get(from).unwrap() {
            if let Some(color) = self.player {
                self.player = Some(color.next());
            } else {
                self.player = Some(c.next());
            }
            self.turn += 1;
            self.board = self.board.play(from, action);
        }
    }

    pub fn restart(&mut self) {
        self.board = make_board();
        self.player = None;
        self.turn = 0;
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