use std::collections::HashMap;

use color_eyre::eyre::Result;
use image::{DynamicImage, RgbaImage, imageops::FilterType};

pub struct CharacterGallery {
    portraits: HashMap<&'static str, CharacterPortrait>,
}

pub struct CharacterPortrait {
    pub reveal: TerminalRaster,
    pub detail: TerminalRaster,
}

pub struct TerminalRaster {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<[u8; 4]>,
}

impl TerminalRaster {
    pub fn pixel(&self, x: u16, y: u16) -> [u8; 4] {
        self.pixels[(y * self.width + x) as usize]
    }
}

impl CharacterGallery {
    pub fn load() -> Result<Self> {
        let mut portraits = HashMap::new();
        for (name, bytes) in PORTRAITS {
            let image = trim_transparent(&image::load_from_memory(bytes)?);
            portraits.insert(
                *name,
                CharacterPortrait {
                    reveal: rasterize(&image, 22, 16),
                    detail: rasterize(&image, 28, 28),
                },
            );
        }
        Ok(Self { portraits })
    }

    pub fn get(&self, name: &str) -> Option<&CharacterPortrait> {
        self.portraits.get(name)
    }
}

fn trim_transparent(image: &DynamicImage) -> RgbaImage {
    let rgba = image.to_rgba8();
    let (mut left, mut top) = (rgba.width(), rgba.height());
    let (mut right, mut bottom) = (0, 0);
    for (x, y, pixel) in rgba.enumerate_pixels() {
        if pixel[3] > 16 {
            left = left.min(x);
            top = top.min(y);
            right = right.max(x);
            bottom = bottom.max(y);
        }
    }
    if left > right || top > bottom {
        return rgba;
    }
    image::imageops::crop_imm(&rgba, left, top, right - left + 1, bottom - top + 1).to_image()
}

fn rasterize(image: &RgbaImage, max_width: u32, max_height: u32) -> TerminalRaster {
    let scale =
        (max_width as f32 / image.width() as f32).min(max_height as f32 / image.height() as f32);
    let width = (image.width() as f32 * scale).round().max(1.0) as u32;
    let height = (image.height() as f32 * scale).round().max(1.0) as u32;
    let resized = image::imageops::resize(image, width, height, FilterType::Nearest);
    TerminalRaster {
        width: width as u16,
        height: height as u16,
        pixels: resized.pixels().map(|pixel| pixel.0).collect(),
    }
}

const PORTRAITS: &[(&str, &[u8])] = &[
    (
        "Astraea, Starbound",
        include_bytes!("../assets/characters/astraea.png"),
    ),
    (
        "Kaelis, Ashen Vanguard",
        include_bytes!("../assets/characters/kaelis.png"),
    ),
    (
        "Seraphine, Verdant Oracle",
        include_bytes!("../assets/characters/seraphine.png"),
    ),
    (
        "Veyra, Stormseeker",
        include_bytes!("../assets/characters/veyra.png"),
    ),
    (
        "Orin, Keeper of Embers",
        include_bytes!("../assets/characters/orin.png"),
    ),
    ("Mira", include_bytes!("../assets/characters/mira.png")),
    ("Thorne", include_bytes!("../assets/characters/thorne.png")),
    ("Lumen", include_bytes!("../assets/characters/lumen.png")),
];
