// Copy paste of https://crates.io/crates/text-to-png
// with options modified to optimize for speed
use derive_new::new;
use fontdb::Database;
use resvg::render_node;
use std::fmt::Display;
use tiny_skia::Pixmap;
use usvg::{FitTo, ImageRendering, NodeExt, Options, TextRendering, Tree};

const DEFAULT_FONT: &[u8] = include_bytes!("resources/CallingCode-Regular.ttf");
const DEFAULT_FONT_NAME: &str = "Calling Code";

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
    /// Red Component
    pub r: u8,

    /// Green Component
    pub g: u8,

    /// Blue Component
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

pub fn render_text_to_png_data(text: String, font_size: u32, color: Color) -> Vec<u8> {
    let content = format!(
        include_str!("resources/template.svg"),
        font_size, color, text
    );

    let tree = Tree::from_str(content.as_str(), &OPTIONS.to_ref()).unwrap();

    let text_node = tree.node_by_id("t").unwrap();

    let size = text_node.calculate_bbox().unwrap();

    let mut pixmap = Pixmap::new(size.width().ceil() as u32, size.height().ceil() as u32).unwrap();

    render_node(&tree, &text_node, FitTo::Original, pixmap.as_mut());
    pixmap.encode_png().unwrap()
}
