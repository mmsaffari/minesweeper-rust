use macroquad::text::{Font, load_ttf_font_from_bytes};

/// The game's fonts. They're compiled into the program (`include_bytes!`),
/// so there's no font file to ship next to the executable, or to lose.
pub struct Fonts {
    /// Numbers, flags and question marks on the board.
    pub cell: Font,
    /// The red 7-segment mine counter and timer (Task 5d).
    pub counter: Font,
}

impl Fonts {
    /// Call once at startup, inside `main` (fonts need the window to exist), never per frame.
    pub fn load() -> Self {
        Self {
            cell: load(include_bytes!("../../assets/fonts/Roboto-Bold.ttf")),
            counter: load(include_bytes!("../../assets/fonts/DSEG7Classic-Bold.ttf")),
        }
    }
}

fn load(bytes: &[u8]) -> Font {
    load_ttf_font_from_bytes(bytes).expect("the embedded font files are valid TTF")
}
