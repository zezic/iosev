use femtovg::{Canvas, Renderer, Paint, Color};

use crate::Fonts;

enum WallTile {
    H,
    V,
    TR,
    BR,
    BL,
    TL,
}

enum WallStyle {
    Simple,
    Round,
}

enum Object {
    Void,
    Matter,
    Wall {
        tile: WallTile,
        style: WallStyle,
    },
    Player,
}

// ╭───╮
// │ @ │
// ╰───╯

impl Object {
    fn chr(&self) -> &str {
        match self {
            Object::Void => "░",
            Object::Matter => "▒",
            Object::Wall {
                tile,
                style,
            } => match tile {
                WallTile::H => "─",
                WallTile::V => "│",
                WallTile::TR => "╮",
                WallTile::BR => "╯",
                WallTile::BL => "╰",
                WallTile::TL => "╭",
            },
            Object::Player => "@",
        }
    }
    fn wall(tile: WallTile) -> Self {
        Self::Wall { tile, style: WallStyle::Round }
    }
    fn wall_hor_chr(&self) -> &str {
        match self {
            Object::Wall { style, .. } => {
                match style {
                    WallStyle::Simple | WallStyle::Round => "─",
                }
            },
            _ => unreachable!()
        }
    }
}

pub struct Engine {
    map: Vec<Object>,
    map_width: usize,
}

impl Engine {
    pub fn new() -> Self {
        use Object::*;
        use WallTile::*;
        Self {
            map: vec![
                Object::wall(TL), Object::wall(H), Object::wall(TR), Void,
                Object::wall(V), Player, Object::wall(V), Void,
                Object::wall(BL), Object::wall(H), Object::wall(BR), Void,
            ],
            map_width: 4,
        }
    }

    pub fn draw<T: Renderer>(&self, cnv: &mut Canvas<T>, fonts: &Fonts) {
        let mut paint = Paint::color(Color::rgb(220, 220, 220));
        let font_size = 80.0;
        const IOSEVKA_HEIGHT_RATIO: f32 = 10.0 / 4.0;
        paint.set_font(&[fonts.ext]);
        paint.set_font_size(font_size);

        // let hor_step = font_size / 2.0;
        let hor_step_ext = font_size * 1.2 / 2.0;
        // let hor_step_sqr = font_size * 1.44 / 2.0;
        let line_height = font_size * IOSEVKA_HEIGHT_RATIO / 2.0;

        let tail = [Object::Void];

        for (y_idx, line) in self.map.chunks(self.map_width).enumerate() {
            for (x_idx, objs) in line.windows(2).enumerate() {
                let mut objs = objs.iter();
                let obj = objs.next().unwrap();
                let next = objs.next().unwrap();
                cnv.fill_text(110.0 + hor_step_ext * x_idx as f32 * 2.0 - hor_step_ext, 110.0 + line_height * y_idx as f32, obj.chr(), paint).expect("Can't fill text");
                if let (Object::Wall { .. }, Object::Wall { .. }) = (obj, next) {
                    cnv.fill_text(110.0 + hor_step_ext * x_idx as f32 * 2.0, 110.0 + line_height * y_idx as f32, obj.wall_hor_chr(), paint).expect("Can't fill text");
                };
            }
        }
    }
}

