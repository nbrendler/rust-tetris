use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};

use crate::piece::{Block, PieceType};

pub struct Assets {
    pink_block_image: graphics::Image,
    purple_block_image: graphics::Image,
    green_block_image: graphics::Image,
    orange_block_image: graphics::Image,
    red_block_image: graphics::Image,
    yellow_block_image: graphics::Image,
    blue_block_image: graphics::Image,
    pub background: graphics::Image,
    pub font: graphics::Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let pink_block_image = graphics::Image::new(ctx, "/Pink.png")?;
        let purple_block_image = graphics::Image::new(ctx, "/Purple.png")?;
        let green_block_image = graphics::Image::new(ctx, "/Green.png")?;
        let orange_block_image = graphics::Image::new(ctx, "/Orange.png")?;
        let red_block_image = graphics::Image::new(ctx, "/Red.png")?;
        let yellow_block_image = graphics::Image::new(ctx, "/Yellow.png")?;
        let blue_block_image = graphics::Image::new(ctx, "/Blue.png")?;
        let background = graphics::Image::new(ctx, "/background.png")?;
        let font = graphics::Font::new(ctx, "/PressStart2P.ttf")?;

        Ok(Assets {
            pink_block_image,
            purple_block_image,
            green_block_image,
            orange_block_image,
            red_block_image,
            yellow_block_image,
            blue_block_image,
            background,
            font,
        })
    }

    pub fn get_image_for_block(&self, b: &Block) -> &graphics::Image {
        match b.piece_type {
            PieceType::IShape => &self.pink_block_image,
            PieceType::OShape => &self.purple_block_image,
            PieceType::LShape => &self.green_block_image,
            PieceType::LShapeInverted => &self.orange_block_image,
            PieceType::RShape => &self.red_block_image,
            PieceType::RShapeInverted => &self.yellow_block_image,
            PieceType::TShape => &self.blue_block_image,
        }
    }
}
