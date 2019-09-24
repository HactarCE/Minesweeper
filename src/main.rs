#![allow(dead_code)]

#[macro_use]
extern crate ndarray;

mod board;
mod render;
mod sprites;
mod ui;
mod utils;

use tetra::{Context, ContextBuilder, State};

use board::Board;

struct GameState {
    board: Board,
    render_state: render::RenderState,
    ui_state: ui::UIState,
    exploded: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let board = Board::make_random((16, 30), 99).unwrap();
        let mut game_state = GameState {
            board: Board::make_empty((1, 1)),
            render_state: render::RenderState::new(ctx)?,
            ui_state: ui::UIState::new(),
            exploded: false,
        };
        game_state.set_board(ctx, board);
        Ok(game_state)
    }
}

impl GameState {
    fn set_board(&mut self, ctx: &mut Context, board: Board) {
        self.board = board;
        self.reset_window_size(ctx);
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.handle_tile_left_click(ctx);
        self.handle_tile_right_click(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        self.draw_borders(ctx)?;
        self.draw_tiles(ctx)?;
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Minesweeper", 1080, 720)
        .show_mouse(true)
        .build()?
        .run_with(GameState::new)
}
