use tetra::{
    input::{self, MouseButton},
    Context,
};

use crate::board::{Pos, Tile, TileState};
use crate::GameStage;
use crate::GameState;

pub struct UIState {
    pub left_clicked_tile: Option<(usize, usize)>,
    pub right_clicked_tile: Option<(usize, usize)>,
    pub face_clicked: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            left_clicked_tile: None,
            right_clicked_tile: None,
            face_clicked: false,
        }
    }
}

impl GameState {
    pub fn handle_face_click(&mut self, ctx: &mut Context) {
        if self.is_mouse_on_face(ctx) {
            if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
                self.ui_state.face_clicked = true;
            } else if input::is_mouse_button_released(ctx, MouseButton::Left) {
                if self.ui_state.face_clicked {
                    self.reset_game(ctx);
                    self.ui_state.face_clicked = false;
                }
            }
        }
    }

    pub fn handle_tile_left_click(&mut self, ctx: &mut Context) -> Vec<Pos> {
        let hover_tile = self.get_tile_at_cursor(ctx);
        if let Some(_) = self.ui_state.left_clicked_tile {
            if input::is_mouse_button_down(ctx, MouseButton::Left) {
                self.ui_state.left_clicked_tile = hover_tile;
            } else if input::is_mouse_button_released(ctx, MouseButton::Left) {
                self.ui_state.left_clicked_tile = None;
                if let Some(hover_tile) = hover_tile {
                    if self.stage == GameStage::Pre {
                        self.board.ensure_safe_start(hover_tile);
                        self.stage = GameStage::Playing;
                    }
                    let updated_squares = self.board.left_click(hover_tile);
                    for pos in updated_squares {
                        if self.board.get_tiles()[pos] == Tile::Mine
                            && self.board.get_tilestates()[pos] == TileState::Uncovered
                        {
                            self.stage = GameStage::Exploded;
                        }
                    }
                }
            }
        } else if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            self.ui_state.left_clicked_tile = hover_tile;
        }
        vec![]
    }

    pub fn handle_tile_right_click(&mut self, ctx: &mut Context) -> Vec<Pos> {
        let hover_tile = self.get_tile_at_cursor(ctx);
        if let Some(_) = self.ui_state.right_clicked_tile {
            if input::is_mouse_button_down(ctx, MouseButton::Right) {
                self.ui_state.right_clicked_tile = hover_tile;
            } else if input::is_mouse_button_released(ctx, MouseButton::Right) {
                self.ui_state.right_clicked_tile = None;
                if let Some(hover_tile) = hover_tile {
                    return self.board.right_click(hover_tile);
                }
            }
        } else if input::is_mouse_button_pressed(ctx, MouseButton::Right) {
            self.ui_state.right_clicked_tile = hover_tile;
        }
        vec![]
    }
}
