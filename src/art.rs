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
    (
        "Vaughn, Violet Oath",
        include_bytes!("../assets/characters/vaughn.png"),
    ),
    (
        "Steven, Azure Shade",
        include_bytes!("../assets/characters/steven.png"),
    ),
    (
        "Cinder, Forgeheart",
        include_bytes!("../assets/characters/cinder.png"),
    ),
    (
        "Sergei, Winterfang",
        include_bytes!("../assets/characters/sergei.png"),
    ),
    ("Zephra", include_bytes!("../assets/characters/zephra.png")),
    ("Neris", include_bytes!("../assets/characters/neris.png")),
    ("Brikka", include_bytes!("../assets/characters/brikka.png")),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_portraits_are_transparent_cutouts() {
        for (name, bytes) in PORTRAITS {
            let image = image::load_from_memory(bytes).unwrap().to_rgba8();
            let corners = [
                image.get_pixel(0, 0),
                image.get_pixel(image.width() - 1, 0),
                image.get_pixel(0, image.height() - 1),
                image.get_pixel(image.width() - 1, image.height() - 1),
            ];
            assert!(
                corners.iter().all(|pixel| pixel[3] == 0),
                "{name} has an opaque corner: {corners:?}"
            );
            assert!(
                image.pixels().any(|pixel| pixel[3] == 0),
                "{name} has no transparency"
            );
            assert!(
                image.pixels().any(|pixel| pixel[3] == 255),
                "{name} has no opaque character pixels"
            );
        }
    }

    #[test]
    fn every_portrait_loads_through_the_gallery_pipeline() {
        let gallery = CharacterGallery::load().unwrap();
        for (name, _) in PORTRAITS {
            let portrait = gallery.get(name).unwrap();
            assert!(portrait.reveal.width >= 8, "{name} reveal is too narrow");
            assert!(portrait.reveal.height >= 8, "{name} reveal is too short");
            assert!(portrait.detail.width >= 8, "{name} detail is too narrow");
            assert!(portrait.detail.height >= 8, "{name} detail is too short");
        }
    }
}
