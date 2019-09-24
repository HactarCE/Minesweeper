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