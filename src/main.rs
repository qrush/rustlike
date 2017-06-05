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
    let mut firstRun = false;

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.flush();

        draw(&mut root, player_x, player_y, ' ');

        if !firstRun {
            draw_player(&mut root, player_x, player_y);
            firstRun = true;
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

fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
    let key = root.wait_for_keypress(true);

    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,
        _ => {},
    }
    false
}
