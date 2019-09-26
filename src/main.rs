#![allow(dead_code)]

#[macro_use]
extern crate ndarray;

mod board;
mod cli;
mod render;
mod sprites;
mod ui;
mod utils;

use tetra::{Context, ContextBuilder, State};

use board::{Board, Difficulty};

#[derive(Debug, PartialEq)]
enum GameStage {
    Pre,
    Playing,
    Exploded,
    Complete,
}

struct GameState {
    board: Board,
    difficulty: Difficulty,
    render_state: render::RenderState,
    ui_state: ui::UIState,
    stage: GameStage,
    seconds: usize,
    ticks: usize,
}

impl GameState {
    pub fn new(ctx: &mut Context, difficulty: Difficulty) -> tetra::Result<Self> {
        match difficulty.new_game() {
            Ok(board) => {
                let mut game_state = GameState {
                    board: Board::make_empty((1, 1)),
                    difficulty: difficulty,
                    render_state: render::RenderState::new(ctx)?,
                    ui_state: ui::UIState::new(),
                    stage: GameStage::Pre,
                    seconds: 0,
                    ticks: 0,
                };
                game_state.set_board(ctx, board);
                Ok(game_state)
            }
            Err(msg) => {
                println!();
                println!("Could not start game: {}", msg);
                println!();
                cli::print_usage();
                std::process::exit(1);
            }
        }
    }
}

impl GameState {
    pub fn reset_game(&mut self, ctx: &mut Context) {
        self.set_board(
            ctx,
            self.difficulty.new_game().unwrap_or_else(|err| panic!(err)),
        );
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
    match cli::get_difficulty_from_options() {
        Ok(difficulty) => {
            let window_size = GameState::get_window_size(&difficulty.size);
            ContextBuilder::new("Minesweeper", window_size.0, window_size.1)
                .show_mouse(true)
                .build()?
                .run_with(|ctx| GameState::new(ctx, difficulty))
        }
        Err(err) => {
            if let Some(s) = err {
                println!("Could not start game: {}", s);
                println!();
            }
            cli::print_usage();
            std::process::exit(1);
        }
    }
}
