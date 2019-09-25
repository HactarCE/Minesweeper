#![allow(dead_code)]

#[macro_use]
extern crate ndarray;

mod board;
mod render;
mod sprites;
mod ui;
mod utils;

use tetra::{Context, ContextBuilder, State};

use board::{Board, Difficulty};

const DIFFICULTY: Difficulty = Difficulty::Beginner;

#[derive(Debug, PartialEq)]
enum GameStage {
    Pre,
    Playing,
    Exploded,
    Complete,
}

struct GameState {
    board: Board,
    render_state: render::RenderState,
    ui_state: ui::UIState,
    stage: GameStage,
    seconds: usize,
    ticks: usize,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mut game_state = GameState {
            board: Board::make_empty((1, 1)),
            render_state: render::RenderState::new(ctx)?,
            ui_state: ui::UIState::new(),
            stage: GameStage::Pre,
            seconds: 0,
            ticks: 0,
        };
        game_state.reset_game(ctx);
        Ok(game_state)
    }
}

impl GameState {
    pub fn reset_game(&mut self, ctx: &mut Context) {
        self.set_board(ctx, DIFFICULTY.new_game().unwrap());
    }

    fn set_board(&mut self, ctx: &mut Context, board: Board) {
        self.board = board;
        self.reset_window_size(ctx);
        self.reset_timer(ctx);
        self.stage = GameStage::Pre;
    }

    fn reset_timer(&mut self, ctx: &mut Context) {
        self.seconds = 0;
        self.reset_ticks(ctx);
    }

    fn reset_ticks(&mut self, ctx: &mut Context) {
        self.ticks = tetra::time::get_tick_rate(ctx).round() as usize;
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.stage == GameStage::Playing {
            if self.ticks <= 1 {
                self.seconds += 1;
                self.reset_ticks(ctx);
            } else {
                self.ticks -= 1;
            }
        }
        if let GameStage::Pre | GameStage::Playing = self.stage {
            self.handle_tile_left_click(ctx);
            self.handle_tile_right_click(ctx);
        }
        if self.stage == GameStage::Playing && self.board.get_safe_squares_left() == 0 {
            self.stage = GameStage::Complete;
        }
        self.handle_face_click(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        self.draw_borders(ctx);
        self.draw_tiles(ctx);
        self.draw_mine_counter(ctx);
        self.draw_timer(ctx);
        self.draw_face(ctx);
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Minesweeper", 1080, 720)
        .show_mouse(true)
        .build()?
        .run_with(GameState::new)
}
