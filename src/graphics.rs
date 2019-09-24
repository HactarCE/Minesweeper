use crate::grid::{Board, Tile, TileState};
use crate::sprites::*;
use crate::utils::*;

use tetra::graphics::{self, texture::Texture, ui::NineSlice, DrawParams, Rectangle, Vec2};
use tetra::Context;

const NINESLICE_VERTICAL_EXTRA: f32 = 36.0;
const OFFSET_GRID: (f32, f32) = (15.0, 51.0);
const OFFSET_MINES_COUNT: (f32, f32) = (15.0, 18.0);
const OFFSET_TIMER: (f32, f32) = (232.0, 18.0);
const TILE_SIZE: f32 = 16.0;
const TOTAL_PADDING: (f32, f32) = (48.0 - 18.0, 84.0 - 18.0);

pub fn get_border_nineslice(ctx: &mut Context) -> tetra::Result<NineSlice> {
    Ok(NineSlice::new(
        Texture::new(ctx, "./resources/borders.png")?,
        0.0,
        0.0,
        Rectangle::new(16.0, 52.0, 16.0, 16.0),
    ))
}

pub fn get_tiles_spritesheet(ctx: &mut Context) -> tetra::Result<Texture> {
    Texture::new(ctx, "./resources/spritemap.png")
}

pub fn reset_window_size(ctx: &mut Context, grid_size: &(usize, usize)) {
    // Set up window.
    let board_size = vec2_from_reverse_usize_tuple(grid_size);
    let window_size = board_size * TILE_SIZE + vec2_from_tuple(&TOTAL_PADDING);
    graphics::set_internal_size(ctx, window_size[0] as i32, window_size[1] as i32);
    tetra::window::set_size(ctx, window_size[0] as i32 * 3, window_size[1] as i32 * 3);
}

pub fn draw_borders(ctx: &mut Context, nineslice: &mut NineSlice) -> tetra::Result {
    // Draw border
    nineslice.set_size(
        graphics::get_internal_width(ctx) as f32,
        graphics::get_internal_height(ctx) as f32 + NINESLICE_VERTICAL_EXTRA,
    );
    graphics::draw(ctx, nineslice, Vec2::new(0.0, 0.0));
    Ok(())
}

pub fn draw_tiles(ctx: &mut Context, board: &Board, spritemap: &Texture) -> tetra::Result {
    // Draw cells
    let (grid_offset_x, grid_offset_y) = OFFSET_GRID;
    for ((y, x), tilestate) in board.get_tilestates().indexed_iter() {
        let sprite_x = grid_offset_x + (TILE_SIZE * x as f32);
        let sprite_y = grid_offset_y + (TILE_SIZE * y as f32);
        let tile_sprite = match tilestate {
            TileState::Hidden => TileSprite::Hidden,
            TileState::Flagged => TileSprite::Flagged,
            TileState::QuestionMark => TileSprite::QuestionMark,
            TileState::Uncovered => match board.get_tiles()[(y, x)] {
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
            spritemap,
            DrawParams::new()
                .position(vec2_from_tuple(&(sprite_x, sprite_y)))
                .clip(tile_sprite.into()),
        )
    }
    Ok(())
}
