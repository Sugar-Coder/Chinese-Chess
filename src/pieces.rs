use crate::{pos::Pos, chess::Board};

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

#[derive(Clone, Copy)]
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

impl Piece {
    pub fn moves(self, board: &Board, from: Pos, color: PlayerColor) -> Vec<Action> {
        match self {
            Self::Jiang => jiang_moves(board, from, color),
            Self::Shi => shi_moves(board, from, color),
            Self::Xiang => xiang_moves(board, from, color),
            Self::Ma => ma_moves(board, from, color),
            Self::Che => che_moves(board, from, color),
            Self::Pao => pao_moves(board, from, color),
            Self::Bing => bing_moves(board, from, color),
        }
    }
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

fn jiang_moves(board: &Board, from: Pos, color: PlayerColor) -> Vec<Action> {
    let dirs = vec![Pos(-1, 0), Pos(1, 0), Pos(0, 1), Pos(0, -1)];
    let mut bound = vec![Pos(3, 0), Pos(5, 2)];
    if color == PlayerColor::Black {
        bound[0].1 += 7;
        bound[1].1 += 7;
    }
    let mut actions = vec![];
    for dir in dirs {
        let to = from + dir;
        if to.in_bound(bound[0], bound[1]) {
            if let Some(grid) = board.get(to) {
                if let Some((c, _)) = grid {
                    if *c != color {
                        actions.push(Action::Take(to));
                    }
                } else {
                    actions.push(Action::Go(to));
                }
            }
        }
    }
    actions
}

fn shi_moves(board: &Board, from: Pos, color: PlayerColor) -> Vec<Action> {
    let dirs = vec![Pos(-1, -1), Pos(1, 1), Pos(-1, 1), Pos(1, -1)];
    let mut bound = vec![Pos(3, 0), Pos(5, 2)];
    if color == PlayerColor::Black {
        bound[0].1 += 7;
        bound[1].1 += 7;
    }
    let mut actions = vec![];
    for dir in dirs {
        let to = from + dir;
        if to.in_bound(bound[0], bound[1]) {
            if let Some(grid) = board.get(to) {
                if let Some((c, _)) = grid {
                    if *c != color {
                        actions.push(Action::Take(to));
                    }
                } else {
                    actions.push(Action::Go(to));
                }
            }
        }
    }
    actions
}

fn xiang_moves(board: &Board, from: Pos, color: PlayerColor) -> Vec<Action> {
    let dirs = vec![Pos(-2, -2), Pos(2, 2), Pos(-2, 2), Pos(2, -2)];
    let mut bound = vec![Pos(0, 0), Pos(8, 4)];
    if color == PlayerColor::Black {
        bound[0].1 += 5;
        bound[1].1 += 5;
    }
    let mut actions = vec![];
    for dir in dirs {
        let to = from + dir;
        if to.in_bound(bound[0], bound[1]) {
            if let Some(Some((_, _))) = board.get(from + Pos(dir.0 / 2, dir.1 / 2)) {
                continue; // blocked
            }
            if let Some(grid) = board.get(to) {
                if let Some((c, _)) = grid {
                    if *c != color {
                        actions.push(Action::Take(to));
                    }
                } else {
                    actions.push(Action::Go(to));
                }
            }
        }
    }
    actions
}

fn ma_moves(board: &Board, from: Pos, color: PlayerColor) -> Vec<Action> {
    let dirs = vec![
        Pos(-1, -2),
        Pos(1, -2),
        Pos(-2, -1),
        Pos(2, -1),
        Pos(-1, 2),
        Pos(1, 2),
        Pos(-2, 1),
        Pos(2, 1),
    ];
    let mut actions = vec![];
    for dir in dirs {
        let to = from + dir;
        if let Some(grid) = board.get(to) {
            let mut blocked = false;
            if dir.0 == 2 || dir.0 == -2 {
                blocked = board.get(from + Pos(dir.0 / 2, 0)).unwrap().is_some();
            } else {
                blocked = board.get(from + Pos(0, dir.1 / 2)).unwrap().is_some();
            }
            if !blocked {
                if let Some((c, _)) = grid {
                    if *c != color {
                        actions.push(Action::Take(to));
                    }
                } else {
                    actions.push(Action::Go(to));
                }
            }
        }
    }
    actions
}

fn che_moves(board: &Board, from: Pos, color: PlayerColor) -> Vec<Action> {
    let dirs = vec![Pos(-1, 0), Pos(1, 0), Pos(0, 1), Pos(0, -1)];
    let mut actions = vec![];
    for dir in dirs {
        let mut to = from + dir;
        while to.in_bound(Pos(0, 0), Pos(8, 9)) {
            if let Some(grid) = board.get(to) {
                if let Some((c, _)) = grid {
                    if *c != color {
                        actions.push(Action::Take(to));
                    }
                    break;
                } else {
                    actions.push(Action::Go(to));
                }
            }
            to = to + dir;
        }
    }
    actions
}

#[derive(PartialEq)]
enum PaoState {
    Jumped,
    Driving,
}

fn pao_moves(board: &Board, from: Pos, color: PlayerColor) -> Vec<Action> {
    let dirs = vec![Pos(-1, 0), Pos(1, 0), Pos(0, 1), Pos(0, -1)];
    let mut actions = vec![];
    for dir in dirs {
        let mut to = from + dir;
        let mut state = PaoState::Driving;
        while let Some(grid) = board.get(to) {
            if let Some((c, _)) = grid {
                if state == PaoState::Jumped {
                    if *c != color {
                        actions.push(Action::Take(to));
                    }
                    break;
                } else {
                    state = PaoState::Jumped;
                }
            } else {
                if state == PaoState::Driving {
                    actions.push(Action::Go(to));
                }
            }
            to = to + dir;
        }
    }
    actions
}

fn bing_moves(board: &Board, from: Pos, color: PlayerColor) -> Vec<Action> {
    let mut dirs = vec![Pos(0, 1), Pos(-1, 0), Pos(1, 0)];
    let mut bound = vec![Pos(0, 0), Pos(8, 4)];
    if color == PlayerColor::Black {
        dirs[0].1 = -1;
        bound[0].1 += 5;
        bound[1].1 += 5;
    }
    let mut actions = vec![];
    for dir in dirs {
        let to = from + dir;
        if let Some(grid) = board.get(to) {
            if let Some((c, _)) = grid {
                if *c != color {
                    actions.push(Action::Take(to));
                }
            } else {
                actions.push(Action::Go(to));
            }
        }
        if from.in_bound(bound[0], bound[1]) {
            break; // in self bound
        }
    }
    actions
}