use crate::types::{Point2, GridPosition};

#[derive(Debug, Copy, Clone)]
pub struct Position {
    xy: Point2,
    grid_pos: GridPosition,
}

fn get_grid_position(p: Point2) -> GridPosition {
    GridPosition::new(p.x.round() as i32, p.y.round() as i32)
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        let p = Point2::new(x, y);
        Position {
            xy: p,
            grid_pos: get_grid_position(p)
        }
    }

    pub fn from_point(p: Point2) -> Position {
        Position::new(p.x, p.y)
    }

    pub fn from_grid_position(p: GridPosition) -> Position {
        Position::new(p.x as f32, p.y as f32)
    }

    pub fn grid_position(&self) -> GridPosition {
        self.grid_pos
    }

    pub fn absolute_position(&self) -> Point2 {
        self.xy
    }

    pub fn move_left(&self) -> Position {
        Position::new(self.xy.x - 1.0, self.xy.y)
    }

    pub fn move_right(&self) -> Position {
        Position::new(self.xy.x + 1.0, self.xy.y)
    }

    pub fn move_down(&self) -> Position {
        Position::new(self.xy.x, (self.xy.y + 1.0).floor())
    }

    pub fn screen_coords(&self, block_size: f32, x_offset: f32, y_offset: f32) -> Point2 {
        let x = block_size * self.grid_pos.x as f32 + x_offset;
        let y = block_size * self.grid_pos.y as f32 + y_offset;

        Point2::new(x, y)
    }

}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.grid_pos == other.grid_pos
    }
}