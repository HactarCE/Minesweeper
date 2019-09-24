#![allow(dead_code)]

#[macro_use]
extern crate ndarray;

mod grid;

use grid::{Tile, TileState};

use tetra::graphics::ui::NineSlice;
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture, Vec2};
use tetra::{Context, ContextBuilder, State};

struct GameState {
    borders_nineslice: NineSlice,
    board: grid::Board,
    spritemap: Texture,
}

enum TileSprite {
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

const OFFSET_MINES_COUNT: (f32, f32) = (15.0, 18.0);
const OFFSET_TIMER: (f32, f32) = (232.0, 18.0);
const OFFSET_GRID: (f32, f32) = (15.0, 51.0);
const TOTAL_PADDING: (f32, f32) = (48.0 - 18.0, 84.0 - 18.0);
const NINESLICE_PADDING: (f32, f32) = (0.0, 36.0);
const TILE_SIZE: f32 = 16.0;

const COLOR_BG: Color = Color::rgb(0.753, 0.753, 0.753); // #c0c0c0

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut board = grid::Board::make_random((16, 30), 99).unwrap();
        board.reveal_all();
        // Return completed GameState.
        Ok(GameState {
            board,
            borders_nineslice: NineSlice::new(
                Texture::new(ctx, "./resources/borders.png")?,
                50.0,
                100.0,
                Rectangle::new(16.0, 52.0, 16.0, 16.0),
            ),
            spritemap: Texture::new(ctx, "./resources/spritemap.png")?,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        // Set up window.
        let board_size = vec2_from_reverse_usize_tuple(self.board.get_size());
        let window_size = board_size * TILE_SIZE + vec2_from_tuple(&TOTAL_PADDING);
        graphics::set_internal_size(ctx, window_size[0] as i32, window_size[1] as i32);
        tetra::window::set_size(ctx, window_size[0] as i32 * 3, window_size[1] as i32 * 3);
        // Draw border
        let nineslice_size = window_size + vec2_from_tuple(&NINESLICE_PADDING);
        self.borders_nineslice
            .set_size(nineslice_size[0], nineslice_size[1]);
        graphics::draw(ctx, &self.borders_nineslice, Vec2::new(0.0, 0.0));
        // Draw cells
        let (grid_offset_x, grid_offset_y) = OFFSET_GRID;
        for ((y, x), tilestate) in self.board.get_tilestates().indexed_iter() {
            let sprite_x = grid_offset_x + (TILE_SIZE * x as f32);
            let sprite_y = grid_offset_y + (TILE_SIZE * y as f32);
            let tile_sprite = match tilestate {
                TileState::Hidden => TileSprite::Hidden,
                TileState::Flagged => TileSprite::Flagged,
                TileState::QuestionMark => TileSprite::QuestionMark,
                TileState::Uncovered => match self.board.get_tiles()[(y, x)] {
                    Tile::Mine => TileSprite::Mine,
                    Tile::Safe(0) => TileSprite::Safe0,
                    Tile::Safe(1) => TileSprite::Safe1,
                    Tile::Safe(2) => TileSprite::Safe2,
                    Tile::Safe(3) => TileSprite::Safe3,
                    Tile::Safe(4) => TileSprite::Safe4,
                    Tile::Safe(5) => TileSprite::Safe5,
                    Tile::Safe(6) => TileSprite::Safe6,
                    Tile::Safe(7) => TileSprite::Safe7,
                    Tile::Safe(8) => TileSprite::Safe8,
                    _ => TileSprite::QuestionMark,
                },
            };
            graphics::draw(
                ctx,
                &self.spritemap,
                DrawParams::new()
                    .position(vec2_from_tuple(&(sprite_x, sprite_y)))
                    .clip(tile_sprite.into()),
            )
        }
        Ok(())
    }
}

fn vec2_from_reverse_usize_tuple(&(y, x): &(usize, usize)) -> Vec2 {
    vec2_from_usize_tuple(&(x, y))
}

fn vec2_from_usize_tuple(tuple: &(usize, usize)) -> Vec2 {
    Vec2::new(tuple.0 as f32, tuple.1 as f32)
}

fn vec2_from_tuple(tuple: &(f32, f32)) -> Vec2 {
    Vec2::new(tuple.0, tuple.1)
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", 1280, 720)
        .show_mouse(true)
        .build()?
        .run_with(GameState::new)
}
