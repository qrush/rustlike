extern crate tcod;

mod rustlike;

use tcod::console::*;
use tcod::colors;
use tcod::colors::Color;
use tcod::input::Key;
use tcod::input::KeyCode::*;

use rustlike::*;
use rustlike::map::*;
use rustlike::object::*;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map) {
    for object in objects {
        object.draw(con)
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = map[x as usize][y as usize].block_sight;
            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }
    // reset
    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
    root.flush();
    for object in objects {
        object.clear(con);
    }
}

fn main() {
    let mut root = Root::initializer()
      .font("arial10x10.png", FontLayout::Tcod)
      .font_type(FontType::Greyscale)
      .size(SCREEN_WIDTH, SCREEN_HEIGHT)
      .title("Rustlike v1")
      .init();
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    con.set_default_foreground(colors::WHITE);

    tcod::system::set_fps(LIMIT_FPS);

    let player = Object::new(0, 0, '@', colors::WHITE);
    let npc = Object::new(10, 10, '&', colors::YELLOW);
    let mut objects = [player, npc];
    let map = make_map();

    while !root.window_closed() {
        render_all(&mut root, &mut con, &mut objects, &map);

        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
    let key = root.wait_for_keypress(true);

    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } | Key { printable: 'k', .. } => {
            player.move_by(0, -1, map);
        }
        Key { code: Down, .. } | Key { printable: 'j', .. } => {
            player.move_by(0, 1, map);
        }
        //Key { printable: 'J', .. } => {
        //    player.move_by(0, SCREEN_HEIGHT - 1);
        //}
        Key { code: Left, .. } | Key { printable: 'h', .. } => {
            player.move_by(-1, 0, map);
        }
        Key { code: Right, .. } | Key { printable: 'l', .. }  => {
            player.move_by(1, 0, map);
        }
        _ => {},
    }
    false
}
