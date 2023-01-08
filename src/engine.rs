use femtovg::{Canvas, Color, Paint, Renderer};
use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};

use crate::Fonts;

#[derive(PartialEq)]
enum WallTile {
    H,
    V,
    TR,
    BR,
    BL,
    TL,
}

#[derive(PartialEq)]
enum WallStyle {
    Simple,
    Round,
}

#[derive(PartialEq)]
enum Obstacle {
    Wall { tile: WallTile, style: WallStyle },
    Door { state: DoorState },
}

fn wall(tile: WallTile) -> Option<Obstacle> {
    Some(Obstacle::Wall {
        tile,
        style: WallStyle::Round,
    })
}

#[derive(PartialEq)]
enum DoorState {
    Opened,
    Closed,
}

trait Chr<T> {
    fn chr(&self) -> &str;
    fn fill_chr<'a>(left: &'a T, right: &'a T) -> Option<&'a str>;
}

impl Chr<Obstacle> for Obstacle {
    fn chr(&self) -> &str {
        match self {
            Obstacle::Wall { tile, style } => match tile {
                WallTile::H => "─",
                WallTile::V => "│",
                WallTile::TR => "╮",
                WallTile::BR => "╯",
                WallTile::BL => "╰",
                WallTile::TL => "╭",
            },
            Obstacle::Door { .. } => "d",
        }
    }

    fn fill_chr<'a>(left: &'a Obstacle, right: &'a Obstacle) -> Option<&'a str> {
        use Obstacle::*;
        match (&left, &right) {
            (Wall { .. }, Wall { .. }) => Some("─"),
            _ => None,
        }
    }
}

#[derive(PartialEq)]
enum Monster {
    Player,
}

impl Chr<Monster> for Monster {
    fn chr(&self) -> &str {
        match self {
            Monster::Player => "@",
        }
    }
    fn fill_chr<'a>(left: &'a Monster, right: &'a Monster) -> Option<&'a str> {
        None
    }
}

struct Layer<T> {
    array: Vec<Option<T>>,
}

impl<T> Layer<T> {
    fn new(array: Vec<Option<T>>) -> Self {
        Self { array }
    }
}

pub struct Engine {
    obstacles: Layer<Obstacle>,
    monsters: Layer<Monster>,
    map_width: usize,
    player_idx: usize,
}

impl Engine {
    pub fn new() -> Self {
        use Monster::*;
        use WallTile::*;
        let obstacles = vec![
            wall(TL),
            wall(H),
            wall(H),
            wall(H),
            wall(TR),
            None,
            wall(V),
            None,
            None,
            None,
            wall(V),
            None,
            wall(V),
            None,
            None,
            None,
            wall(V),
            None,
            wall(V),
            None,
            None,
            None,
            wall(V),
            None,
            wall(BL),
            wall(H),
            wall(H),
            wall(H),
            wall(BR),
            None,
        ];

        let monsters = vec![
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Player),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let player_idx = monsters.iter().position(|obj| *obj == Some(Player)).expect("No player");

        Self {
            obstacles: Layer::new(obstacles),
            monsters: Layer::new(monsters),
            map_width: 6,
            player_idx,
        }
    }

    pub fn draw<R: Renderer>(&self, cnv: &mut Canvas<R>, fonts: &Fonts) {
        let mut paint = Paint::default();
        let font_size = 80.0;
        paint.set_font(&[fonts.ext]);
        paint.set_font_size(font_size);

        paint.set_color(Color::rgb(160, 160, 160));
        Self::draw_layer(&self.obstacles, self.map_width, cnv, paint);

        paint.set_color(Color::rgb(220, 220, 220));
        Self::draw_layer(&self.monsters, self.map_width, cnv, paint);
    }

    fn draw_layer<R: Renderer, T: Chr<T>>(layer: &Layer<T>, map_width: usize, cnv: &mut Canvas<R>, paint: Paint) {
        const IOSEVKA_HEIGHT_RATIO: f32 = 10.0 / 4.0;
        // let hor_step = font_size / 2.0;
        let hor_step_ext = paint.font_size() * 1.2 / 2.0;
        // let hor_step_sqr = font_size * 1.44 / 2.0;
        let line_height = paint.font_size() * IOSEVKA_HEIGHT_RATIO / 2.0;

        for (y_idx, line) in layer.array.chunks(map_width).enumerate() {
            for (x_idx, objs) in line.windows(2).enumerate() {
                let mut objs = objs.iter();

                let obj = objs.next().unwrap();
                let next = objs.next().unwrap();

                if obj.is_none() {
                    continue;
                }

                let obj = obj.as_ref().unwrap();
                cnv.fill_text(
                    110.0 + hor_step_ext * x_idx as f32 * 2.0 - hor_step_ext,
                    110.0 + line_height * y_idx as f32,
                    obj.chr(),
                    paint,
                )
                .expect("Can't fill text");

                if next.is_none() {
                    continue;
                }

                let next = next.as_ref().unwrap();
                if let Some(chr) = T::fill_chr(&obj, &next) {
                    cnv.fill_text(
                        110.0 + hor_step_ext * x_idx as f32 * 2.0,
                        110.0 + line_height * y_idx as f32,
                        chr,
                        paint,
                    )
                    .expect("Can't fill text");
                }
            }
        }
    }

    pub fn on_keyboard_input(&mut self, input: &KeyboardInput) {
        if input.state != ElementState::Pressed {
            return;
        }
        if let Some(keycode) = input.virtual_keycode {
            use VirtualKeyCode::*;
            let cmd = match keycode {
                Left | R => MoveCmd::Left,
                Up | F => MoveCmd::Up,
                Right | T => MoveCmd::Right,
                Down | S => MoveCmd::Down,
                _ => return,
            };
            self.on_move_cmd(cmd);
        }
    }

    fn on_move_cmd(&mut self, cmd: MoveCmd) {
        use Monster::*;
        let new_player_idx = (self.player_idx as i32 + match cmd {
            MoveCmd::Left => -1,
            MoveCmd::Right => 1,
            _ => return
        }) as usize;
        if self.obstacles.array[new_player_idx].is_some() {
            return;
        }
        self.monsters.array[self.player_idx] = None;
        self.monsters.array[new_player_idx] = Some(Player);
        self.player_idx = new_player_idx;
    }
}

enum MoveCmd {
    Left,
    Up,
    Right,
    Down,
}
