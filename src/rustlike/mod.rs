pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;
pub const LIMIT_FPS: i32 = 20;

mod tile;
pub use self::tile::Tile;

mod map;
pub use self::map::Map;
pub use self::map::make_map;

mod object;
pub use self::object::Object;
