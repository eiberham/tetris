//! Tetromino interface.
//!
//! Provides an interface and some enums to be used
//! by blocks.
//!

use ggez::{
    graphics::{ Color, Canvas, Text, DrawParam },
    Context, GameError
};
use rand::{
    distributions::{ Distribution, Standard },
    Rng
};
use core::fmt::Debug;

use crate::square::*;
use crate::board::Board;

/// Trait that defines the tetromino's behaviour
pub trait Tetromino {
  fn rotate(&mut self);
  fn move_left(&mut self);
  fn move_right(&mut self);
  fn move_down(&mut self);
  fn drop(&mut self, board: Board);
  fn draw(
    &mut self,
    canvas: &mut Canvas,
    context: &mut Context) -> Result<(), GameError>;
}

/// Different kind of shapes for tetrominoes
#[derive(Debug, Copy, Clone, smart_default::SmartDefault, PartialEq )]
pub enum Shape {
  #[default] I, O, T, S, Z, J, L
}

impl Shape {
    pub fn color(&self) -> Color {
        match &self {
            Shape::L => Color::BLUE,
            Shape::J => Color::YELLOW,
            Shape::T => Color::GREEN,
            Shape::I => Color::MAGENTA,
            Shape::Z => Color::WHITE,
            Shape::S => Color::RED,
            Shape::O => Color::CYAN
        }
    }

    pub fn matrix(&self) -> [[[u8; 4]; 4]; 4] {
        match &self {
            Shape::L => square[0],
            Shape::J => square[1],
            Shape::T => square[2],
            Shape::I => square[3],
            Shape::Z => square[4],
            Shape::S => square[5],
            Shape::O => square[6]
        }
    }

    pub fn current(&self, orientation: Orientation) -> usize {
        match orientation {
            Orientation::Up => 0,
            Orientation::Right => 1,
            Orientation::Down => 2,
            Orientation::Left => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
  pub x: f32,
  pub y: f32
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Orientations that could potentially change
/// the tetromino to
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    L,
    R,
    D
}

/// In order to get a random shape with rand::random()
impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0..=6) {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::S,
            4 => Shape::Z,
            5 => Shape::J,
            6 => Shape::L,
            _ => Shape::I
        }
    }
}

pub trait DrawableText {
    fn draw_text(
        &mut self,
        canvas: &mut Canvas, 
        text: &str, 
        point: [f32; 2], 
        scale: f32, 
        color: Color
    ) {
        let mut text = Text::new(text.to_string());
        text.set_font("arcade").set_scale(scale);
        canvas.draw(
            &text,
            DrawParam::from(point).color(color),
        );
    }
}

