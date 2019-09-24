#![allow(dead_code)]

#[macro_use]
extern crate ndarray;

mod graphics;
mod grid;
mod sprites;
mod utils;

use tetra::graphics::NineSlice;
use tetra::graphics::Texture;
use tetra::{Context, ContextBuilder, State};

struct GameState {
    board: grid::Board,
    borders_nineslice: NineSlice,
    tiles_spritemap: Texture,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut board = grid::Board::make_random((16, 30), 99).unwrap();
        // board.reveal_all();
        Ok(GameState {
            board,
            borders_nineslice: graphics::get_border_nineslice(ctx)?,
            tiles_spritemap: graphics::get_tiles_spritesheet(ctx)?,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::reset_window_size(ctx, self.board.get_size());
        graphics::draw_borders(ctx, &mut self.borders_nineslice)?;
        graphics::draw_tiles(ctx, &self.board, &self.tiles_spritemap)?;
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", 1280, 720)
        .show_mouse(true)
        .build()?
        .run_with(GameState::new)
}
