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
    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = 0;
    let mut player_y = 0;
    let mut first_run = false;

    while !root.window_closed() {
        con.set_default_foreground(colors::WHITE);
        root.flush();

        draw(&mut con, player_x, player_y, ' ');
        if !first_run {
            draw_player(&mut con, player_x, player_y);
            first_run = true;
        } else {
            let exit = handle_keys(&mut root, &mut player_x, &mut player_y);
            draw_player(&mut con, player_x, player_y);

            if exit {
                break
            }
        }
        blit(&mut con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
    }
}

fn draw(con: &mut Offscreen, x: i32, y: i32, who: char) {
    con.put_char(x, y, who, BackgroundFlag::None);
}

fn draw_player(con: &mut Offscreen, player_x: i32, player_y: i32) {
    draw(con, player_x, player_y, '@');
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
        Key { code: Up, .. } | Key { printable: 'k', .. } => {
            move_position(*player_y - 1, player_y, SCREEN_HEIGHT);
        }
        Key { code: Down, .. } | Key { printable: 'j', .. } => {
            move_position(*player_y + 1, player_y, SCREEN_HEIGHT);
        }
        Key { printable: 'J', .. } => {
            move_position(SCREEN_HEIGHT - 1, player_y, SCREEN_HEIGHT);
        }
        Key { code: Left, .. } | Key { printable: 'h', .. } => {
            move_position(*player_x - 1, player_x, SCREEN_HEIGHT);
        }
        Key { code: Right, .. } | Key { printable: 'l', .. }  => {
            move_position(*player_x + 1, player_x, SCREEN_HEIGHT);
        }
        _ => {},
    }
    false
}
