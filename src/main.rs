extern crate tcod;

use tcod::console::*;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

fn main() {
    let mut root = Root::initializer()
      .font("arial10x10.png", FontLayout::Tcod)
      .font_type(FontType::Greyscale)
      .size(SCREEN_WIDTH, SCREEN_HEIGHT)
      .title("Rustlike v1")
      .init();

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = 0;
    let mut player_y = 0;
    let mut first_run = false;

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.flush();

        draw(&mut root, player_x, player_y, ' ');
        if !first_run {
            draw_player(&mut root, player_x, player_y);
            first_run = true;
        } else {
            let exit = handle_keys(&mut root, &mut player_x, &mut player_y);
            draw_player(&mut root, player_x, player_y);

            if exit {
                break
            }
        }
    }
}

fn draw(root: &mut Root, x: i32, y: i32, who: char) {
    root.put_char(x, y, who, BackgroundFlag::None);
}

fn draw_player(root: &mut Root, player_x: i32, player_y: i32) {
    draw(root, player_x, player_y, '@');
}

fn move_position(new_pos: i32, pos: &mut i32, max_pos: i32) {
    if max_pos >= new_pos && new_pos >= 0 {
        *pos = new_pos;
    }
}

fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
    let key = root.wait_for_keypress(true);

    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => {
            move_position(*player_y - 1, player_y, SCREEN_HEIGHT);
        }
        Key { code: Down, .. } => {
            move_position(*player_y + 1, player_y, SCREEN_HEIGHT);
        }
        Key { code: Left, .. } => {
            move_position(*player_x - 1, player_x, SCREEN_HEIGHT);
        }
        Key { code: Right, .. } => {
            move_position(*player_x + 1, player_x, SCREEN_HEIGHT);
        }
        _ => {},
    }
    false
}
