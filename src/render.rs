use std::convert::TryInto;
use tetra::graphics::{self, texture::Texture, ui::NineSlice, DrawParams, Rectangle, Vec2};
use tetra::input::MouseButton;
use tetra::Context;

use crate::board::{Tile, TileState};
use crate::sprites::*;
use crate::utils::*;
use crate::GameStage;
use crate::GameState;

const NINESLICE_VERTICAL_EXTRA: f32 = 36.0;
const OFFSET_FACE: (f32, f32) = (-13.0, 15.0);
const OFFSET_MINES_COUNT: (f32, f32) = (15.0, 18.0);
const OFFSET_TIMER: (f32, f32) = (-(13.0 * 3.0 + 15.0), 18.0);
const TILE_OFFSET_X: f32 = 15.0;
const TILE_OFFSET_Y: f32 = 51.0;
const TILE_SIZE: f32 = 16.0;
const TOTAL_PADDING: (f32, f32) = (48.0 - 18.0, 84.0 - 18.0);

pub struct RenderState {
    borders_nineslice: NineSlice,
    spritemap: Texture,
    face_rectangle: Rectangle,
}

impl RenderState {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            borders_nineslice: NineSlice::new(
                Texture::new(ctx, "./resources/borders.png")?,
                0.0,
                0.0,
                Rectangle::new(16.0, 52.0, 16.0, 16.0),
            ),
            spritemap: Texture::new(ctx, "./resources/spritemap.png")?,
            face_rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
        })
    }
}

impl GameState {
    pub fn reset_window_size(&self, ctx: &mut Context) {
        let board_size = vec2_from_reverse_usize_tuple(self.board.get_size());
        let window_size = board_size * TILE_SIZE + vec2_from_tuple(&TOTAL_PADDING);
        graphics::set_internal_size(ctx, window_size[0] as i32, window_size[1] as i32);
        tetra::window::set_size(ctx, window_size[0] as i32 * 3, window_size[1] as i32 * 3);
    }

    pub fn draw_borders(&mut self, ctx: &mut Context) {
        self.render_state.borders_nineslice.set_size(
            graphics::get_internal_width(ctx) as f32,
            graphics::get_internal_height(ctx) as f32 + NINESLICE_VERTICAL_EXTRA,
        );
        graphics::draw(
            ctx,
            &self.render_state.borders_nineslice,
            Vec2::new(0.0, 0.0),
        );
    }

    pub fn draw_tiles(&self, ctx: &mut Context) {
        let mut clicked_tile: Option<(usize, usize)> = None;
        if let GameStage::Pre | GameStage::Playing = self.stage {
            if tetra::input::is_mouse_button_down(ctx, tetra::input::MouseButton::Left) {
                clicked_tile = self.get_tile_at_cursor(ctx);
            }
        }
        for (tile_pos, tilestate) in self.board.get_tilestates().indexed_iter() {
            let tile = self.board.get_tiles()[tile_pos];
            let tile_sprite = match tilestate {
                TileState::Hidden => {
                    if self.stage == GameStage::Exploded && tile == Tile::Mine {
                        TileSprite::Mine
                    } else if clicked_tile == Some(tile_pos) {
                        TileSprite::HiddenClick
                    } else {
                        TileSprite::Hidden
                    }
                }
                TileState::Flagged => {
                    if self.stage == GameStage::Exploded && tile != Tile::Mine {
                        TileSprite::IncorrectFlag
                    } else {
                        TileSprite::Flagged
                    }
                }
                TileState::QuestionMark => {
                    if self.stage == GameStage::Exploded && tile == Tile::Mine {
                        TileSprite::MineExploded
                    } else if clicked_tile == Some(tile_pos) {
                        TileSprite::QuestionMarkClick
                    } else {
                        TileSprite::QuestionMark
                    }
                }
                TileState::Uncovered => match tile {
                    Tile::Mine => TileSprite::MineExploded,
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
                &self.render_state.spritemap,
                DrawParams::new()
                    .position(vec2_from_tuple(&self.get_tile_display_pos(tile_pos)))
                    .clip(tile_sprite.into()),
            )
        }
    }

    fn get_tile_display_pos(&self, (y, x): (usize, usize)) -> (f32, f32) {
        (
            TILE_SIZE * x as f32 + TILE_OFFSET_X,
            TILE_SIZE * y as f32 + TILE_OFFSET_Y,
        )
    }

    pub fn get_tile_at_cursor(&self, ctx: &Context) -> Option<(usize, usize)> {
        let mouse_pos = tetra::input::get_mouse_position(ctx);
        let x = ((mouse_pos[0] - TILE_OFFSET_X) / TILE_SIZE).floor() as usize;
        let y = ((mouse_pos[1] - TILE_OFFSET_Y) / TILE_SIZE).floor() as usize;
        let &(max_y, max_x) = self.board.get_size();
        if y < max_y && x < max_x {
            Some((y, x))
        } else {
            None
        }
    }

    pub fn draw_mine_counter(&self, ctx: &mut Context) {
        self.draw_triple_7seg(
            ctx,
            vec2_from_tuple(&OFFSET_MINES_COUNT),
            self.board.get_flags_left().try_into().unwrap_or(0),
        );
    }

    pub fn draw_timer(&self, ctx: &mut Context) {
        self.draw_triple_7seg(
            ctx,
            vec2_from_tuple(&OFFSET_TIMER)
                + Vec2::new(tetra::graphics::get_internal_width(ctx) as f32, 0.0),
            self.seconds,
        );
    }

    fn draw_triple_7seg(&self, ctx: &mut Context, mut position: Vec2, mut number: usize) {
        for _ in 0..3 {
            graphics::draw(
                ctx,
                &self.render_state.spritemap,
                DrawParams::new()
                    .position(position)
                    .clip(get_7seg_sprite_clip((number / 100) % 10)),
            );
            number *= 10;
            position += Vec2::new(13.0, 0.0);
        }
    }

    pub fn draw_face(&mut self, ctx: &mut Context) {
        let x_center = tetra::graphics::get_internal_width(ctx) as f32 / 2.0;
        let position = Vec2::new(x_center, 0.0) + vec2_from_tuple(&OFFSET_FACE);
        self.render_state.face_rectangle = Rectangle::new(position[0], position[1], 26.0, 26.0);
        let sprite = if self.is_mouse_on_face(ctx)
            && tetra::input::is_mouse_button_down(ctx, MouseButton::Left)
            && self.ui_state.face_clicked
        {
            FaceSprite::HappyPressed
        } else {
            match self.stage {
                GameStage::Pre | GameStage::Playing => match self.ui_state.left_clicked_tile {
                    None => FaceSprite::Happy,
                    Some(pos) => match self.board.get_tilestates()[pos] {
                        TileState::Hidden | TileState::QuestionMark => FaceSprite::Scared,
                        _ => FaceSprite::Happy,
                    },
                },
                GameStage::Exploded => FaceSprite::Dead,
                GameStage::Complete => FaceSprite::Cool,
            }
        };
        graphics::draw(
            ctx,
            &self.render_state.spritemap,
            DrawParams::new().position(position).clip(sprite.into()),
        )
    }

    pub fn is_mouse_on_face(&self, ctx: &mut Context) -> bool {
        let rect = self.render_state.face_rectangle;
        let mouse_pos = tetra::input::get_mouse_position(ctx);
        let (mouse_x, mouse_y) = (mouse_pos.x, mouse_pos.y);
        rect.x <= mouse_x
            && mouse_x < rect.x + rect.width
            && rect.y <= mouse_y
            && mouse_y < rect.y + rect.height
    }
}
