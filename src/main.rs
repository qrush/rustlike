extern crate tcod;

use tcod::console::*;
use tcod::colors;
use tcod::colors::Color;
use tcod::input::Key;
use tcod::input::KeyCode::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
const LIMIT_FPS: i32 = 20;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

#[derive(Debug)]
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

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;
        let blocked = map[new_x as usize][new_y as usize].blocked;

        if blocked {
            return
        }

        if new_x >= 0 && new_x <= SCREEN_WIDTH - 1 {
            self.x += dx;
        }

        if new_y >= 0 && new_y <= SCREEN_HEIGHT - 1 {
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

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { blocked: false, block_sight: false }
    }

    pub fn wall() -> Self {
        Tile { blocked: true, block_sight: true }
    }
}

type Map = Vec<Vec<Tile>>;

fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();
    map
}

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
