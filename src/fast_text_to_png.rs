// Mostly stolen from https://crates.io/crates/text-to-png
use derive_new::new;
use fontdb::Database;
use resvg::render_node;
use std::{collections::HashMap, fmt::Display};
use tiny_skia::{Pixmap, PremultipliedColorU8};
use usvg::{FitTo, ImageRendering, NodeExt, Options, TextRendering, Tree};

const DEFAULT_FONT: &[u8] = include_bytes!("resources/CallingCode-Regular.ttf");
const DEFAULT_FONT_NAME: &str = "Calling Code";
const FONT_SIZE: u32 = 15;

lazy_static::lazy_static! {
    static ref DEFAULT_FONT_DB : Database = create_default_font_db();
}

lazy_static::lazy_static! {
    static ref OPTIONS: Options = Options {
        font_family: DEFAULT_FONT_NAME.into(),
        fontdb: DEFAULT_FONT_DB.clone(),
        text_rendering: TextRendering::OptimizeSpeed,
        image_rendering: ImageRendering::OptimizeSpeed,
        ..Options::default()
    };
}

fn create_default_font_db() -> Database {
    let mut result = Database::new();

    result.load_font_data(DEFAULT_FONT.to_vec());

    result
}

/// Representation of a RGB color
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, new)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:02X?}{:02X?}{:02X?}",
            self.r, self.g, self.b
        ))
    }
}

impl From<u32> for Color {
    /// This will create a color from the lower 24bits of the given u32 with
    /// red being the most significant
    fn from(mut value: u32) -> Self {
        let b = (value & 0xFF) as u8;
        value >>= 8;
        let g = (value & 0xFF) as u8;
        value >>= 8;
        let r = (value & 0xFF) as u8;

        Color { r, g, b }
    }
}

pub struct SmartPixmap {
    pixmap: Pixmap,
    pixel_positions: Vec<usize>,
}

impl From<Pixmap> for SmartPixmap {
    fn from(pixmap: Pixmap) -> Self {
        let pixel_positions = pixmap
            .pixels()
            .iter()
            .enumerate()
            .filter(|(_, p)| p.alpha() == 255)
            .map(|(i, _)| i)
            .collect();
        Self {
            pixmap,
            pixel_positions,
        }
    }
}

impl SmartPixmap {
    pub fn get_colored_png(&self, color: Color) -> Vec<u8> {
        let mut pixmap = self.pixmap.clone();
        let pixels = pixmap.pixels_mut();
        let c = PremultipliedColorU8::from_rgba(color.r, color.g, color.b, 255).unwrap();
        for &i in &self.pixel_positions {
            pixels[i] = c;
        }
        pixmap.encode_png().unwrap()
    }
}

fn black_pixmap_for_text(text: String) -> Pixmap {
    let content = format!(
        include_str!("resources/template.svg"),
        FONT_SIZE,
        Color { r: 0, g: 0, b: 0 },
        text
    );

    let tree = Tree::from_str(content.as_str(), &OPTIONS.to_ref()).unwrap();

    let text_node = tree.node_by_id("t").unwrap();

    let size = text_node.calculate_bbox().unwrap();

    let mut pixmap = Pixmap::new(size.width().ceil() as u32, size.height().ceil() as u32).unwrap();

    render_node(&tree, &text_node, FitTo::Original, pixmap.as_mut());

    pixmap
}

pub fn get_num_to_black_pixmaps(max_num: usize) -> HashMap<usize, SmartPixmap> {
    (0..=max_num)
        .map(|n| black_pixmap_for_text(n.to_string()).into())
        .enumerate()
        .collect()
}
