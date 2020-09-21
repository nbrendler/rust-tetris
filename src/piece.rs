use std::f32::consts::{PI, FRAC_PI_2};
use rand::{Rng};

use crate::position::Position;
use crate::types::{Rot2, Vec2, GridPosition};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    IShape,
    LShape,
    LShapeInverted,
    RShape,
    RShapeInverted,
    OShape,
    TShape
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Block {
    pub piece_type: PieceType,
    pub pos: Position,
}

impl Block {
    pub fn from_piece(p: &Piece, pos: Position) -> Block {
        Block {
            piece_type: p.piece_type,
            pos,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub pos: Position,
    pub facing: Direction,
    pub velocity: Vec2,
    pub landed: bool,
    block_positions: Vec<GridPosition>,
}

impl Piece {
    pub fn rotate_cw(&mut self) {
        self.facing = match self.piece_type {
            PieceType::OShape => self.facing,
            _ => {
                match self.facing {
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                    Direction::North => Direction::East,
                }
            }

        };
    }

    pub fn rotate_ccw(&mut self) {
        self.facing = match self.piece_type {
            PieceType::OShape => self.facing,
            _ => {
                match self.facing {
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                    Direction::North => Direction::West,
                }
            }

        };
    }

    pub fn block_position(&self, pos: &Position, rel_p: &GridPosition) -> Block {
        let rotation = match self.facing {
            Direction::East => Rot2::new(0.0),
            Direction::South => Rot2::new(-1. * FRAC_PI_2),
            Direction::West => Rot2::new(-1. * PI),
            Direction::North => Rot2::new(-3. * FRAC_PI_2),
        };

        let piece_pos = pos.absolute_position();
        let block_pos = Position::from_grid_position(*rel_p).absolute_position();
        
        Block {
            piece_type: self.piece_type,
            pos: Position::from_point(piece_pos + rotation * block_pos.coords)
        }
    }

    pub fn get_blocks(&self, pos: &Position) -> Vec<Block> {
        self.block_positions.iter().map(|b_pos| {
            self.block_position(pos, &b_pos)
        }).collect()
    }
}

fn get_block_positions(piece_type: PieceType) -> Vec<GridPosition> {
    match piece_type {
        PieceType::IShape => vec![GridPosition::new(-1, 0), GridPosition::new(0, 0),GridPosition::new(1, 0), GridPosition::new(2, 0)],
        PieceType::OShape => vec![GridPosition::new(0, 0), GridPosition::new(0, -1),GridPosition::new(1, 0), GridPosition::new(1, -1)],
        PieceType::LShape => vec![GridPosition::new(-1, 0), GridPosition::new(0, 0),GridPosition::new(1, 0), GridPosition::new(1, -1)],
        PieceType::LShapeInverted => vec![GridPosition::new(-1, 0), GridPosition::new(0, 0),GridPosition::new(1, 0), GridPosition::new(1, 1)],
        PieceType::RShape => vec![GridPosition::new(1, 0), GridPosition::new(0, 0),GridPosition::new(0, -1), GridPosition::new(-1, -1)],
        PieceType::RShapeInverted => vec![GridPosition::new(-1, 0), GridPosition::new(0, 0),GridPosition::new(0, -1), GridPosition::new(1, -1)],
        PieceType::TShape => vec![GridPosition::new(-1, 0), GridPosition::new(0, 0),GridPosition::new(1, 0), GridPosition::new(0, -1)],
    }
}

pub fn create_random_piece(velocity: f32) -> Piece {
    let piece_type: PieceType;
    let mut rng = rand::thread_rng();
    let r0: u32 = rng.gen_range(0, 7);
    piece_type = match r0 {
        0 => PieceType::IShape,
        1 => PieceType::LShape,
        2 => PieceType::LShapeInverted,
        3 => PieceType::RShape,
        4 => PieceType::RShapeInverted,
        5 => PieceType::OShape,
        6 => PieceType::TShape,
        _ => panic!("generated a number out of range")
    };

    let p = Piece {
        piece_type,
        pos: Position::new(4., 1.),
        facing: Direction::North,
        landed: false,
        velocity: Vec2::new(0., velocity),
        block_positions: get_block_positions(piece_type),
    };

    p
}
