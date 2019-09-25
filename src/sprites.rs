use tetra::graphics::Rectangle;

pub enum TileSprite {
    Hidden,
    HiddenClick,
    Flagged,
    QuestionMark,
    QuestionMarkClick,
    Mine,
    MineExploded,
    IncorrectFlag,
    Safe0,
    Safe1,
    Safe2,
    Safe3,
    Safe4,
    Safe5,
    Safe6,
    Safe7,
    Safe8,
}

impl Into<Rectangle> for TileSprite {
    fn into(self) -> Rectangle {
        let (u, v) = match self {
            TileSprite::Hidden => (2.0, 53.0),
            TileSprite::HiddenClick => (19.0, 53.0),
            TileSprite::Flagged => (36.0, 53.0),
            TileSprite::QuestionMark => (53.0, 53.0),
            TileSprite::QuestionMarkClick => (70.0, 53.0),
            TileSprite::Mine => (87.0, 53.0),
            TileSprite::MineExploded => (104.0, 53.0),
            TileSprite::IncorrectFlag => (121.0, 53.0),
            TileSprite::Safe0 => (19.0, 53.0), // Same as HiddenClick
            TileSprite::Safe1 => (2.0, 70.0),
            TileSprite::Safe2 => (19.0, 70.0),
            TileSprite::Safe3 => (36.0, 70.0),
            TileSprite::Safe4 => (53.0, 70.0),
            TileSprite::Safe5 => (70.0, 70.0),
            TileSprite::Safe6 => (87.0, 70.0),
            TileSprite::Safe7 => (104.0, 70.0),
            TileSprite::Safe8 => (121.0, 70.0),
        };
        Rectangle::new(u, v, 16.0, 16.0)
    }
}

pub fn get_7seg_sprite_clip(digit: usize) -> Rectangle {
    let sprite_id = (digit + 9) % 10;
    Rectangle::new(sprite_id as f32 * 14.0 + 2.0, 2.0, 13.0, 23.0)
}

#[derive(Debug)]
pub enum FaceSprite {
    Happy,
    HappyPressed,
    Scared,
    Cool,
    Dead,
}

impl Into<Rectangle> for FaceSprite {
    fn into(self) -> Rectangle {
        let (u, v) = (
            match self {
                FaceSprite::Happy => 2.0,
                FaceSprite::HappyPressed => 29.0,
                FaceSprite::Scared => 56.0,
                FaceSprite::Cool => 83.0,
                FaceSprite::Dead => 110.0,
            },
            26.0,
        );
        Rectangle::new(u, v, 26.0, 26.0)
    }
}
