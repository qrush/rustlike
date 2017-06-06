extern crate tcod;

use tcod::console::*;
use tcod::colors;
use tcod::colors::Color;
use tcod::input::Key;
use tcod::input::KeyCode::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

struct Object {
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

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        if self.x + dx >= 0 && self.x + dx <= SCREEN_WIDTH - 1 {
            self.x += dx;
        }

        if self.y + dy >= 0 && self.y + dy <= SCREEN_HEIGHT - 1{
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

fn main() {
    let mut root = Root::initializer()
      .font("arial10x10.png", FontLayout::Tcod)
      .font_type(FontType::Greyscale)
      .size(SCREEN_WIDTH, SCREEN_HEIGHT)
      .title("Rustlike v1")
      .init();
    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    con.set_default_foreground(colors::WHITE);

    tcod::system::set_fps(LIMIT_FPS);

    let player = Object::new(0, 0, '@', colors::WHITE);
    let npc = Object::new(10, 10, '&', colors::YELLOW);
    let mut objects = [player, npc];

    while !root.window_closed() {
        for object in &objects {
            object.draw(&mut con);
        }
        blit(&mut con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush();

        for object in &objects {
            object.clear(&mut con);
        }

        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player);
        if exit {
            break
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut Object) -> bool {
    let key = root.wait_for_keypress(true);

    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } | Key { printable: 'k', .. } => {
            player.move_by(0, -1);
        }
        Key { code: Down, .. } | Key { printable: 'j', .. } => {
            player.move_by(0, 1);
        }
        //Key { printable: 'J', .. } => {
        //    player.move_by(0, SCREEN_HEIGHT - 1);
        //}
        Key { code: Left, .. } | Key { printable: 'h', .. } => {
            player.move_by(-1, 0);
        }
        Key { code: Right, .. } | Key { printable: 'l', .. }  => {
            player.move_by(1, 0);
        }
        _ => {},
    }
    false
}
