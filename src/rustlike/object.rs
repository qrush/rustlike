extern crate tcod;

use tcod::console::*;
use tcod::colors::Color;
use rustlike::*;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    display: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, display: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            display: display,
            color: color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if new_x >= 0 &&
           new_x <= SCREEN_WIDTH - 1 &&
           new_y >= 0 &&
           new_y <= SCREEN_HEIGHT - 1 &&
           !map[new_x as usize][new_y as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.display, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}
