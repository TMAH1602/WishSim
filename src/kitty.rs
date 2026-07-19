use std::io::{self, Write};

use color_eyre::eyre::Result;
use ratatui::layout::Rect;

const IMAGE_ID: u32 = 9_173;
const CHUNK_SIZE: usize = 4_096;

#[derive(Default)]
pub struct GraphicsRenderer {
    current: Option<(String, Rect)>,
}

impl GraphicsRenderer {
    pub fn sync(&mut self, portrait: Option<(&str, Rect)>) -> Result<()> {
        let next = portrait.map(|(name, area)| (name.to_owned(), area));
        if self.current == next {
            return Ok(());
        }
        self.clear()?;
        if let Some((name, area)) = &next
            && let Some(bytes) = portrait_bytes(name)
        {
            display_png(&mut io::stdout(), bytes, *area)?;
        }
        self.current = next;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        if self.current.is_some() {
            let mut stdout = io::stdout();
            write!(stdout, "\x1b_Ga=d,d=I,i={IMAGE_ID},q=1\x1b\\")?;
            stdout.flush()?;
            self.current = None;
        }
        Ok(())
    }
}

fn display_png(writer: &mut impl Write, png: &[u8], area: Rect) -> Result<()> {
    let payload = base64_encode(png);
    write!(writer, "\x1b[{};{}H", area.y + 1, area.x + 1)?;
    for (index, chunk) in payload.as_bytes().chunks(CHUNK_SIZE).enumerate() {
        let more = usize::from((index + 1) * CHUNK_SIZE < payload.len());
        if index == 0 {
            write!(
                writer,
                "\x1b_Ga=T,f=100,t=d,i={IMAGE_ID},c={},r={},C=1,q=1,m={more};",
                area.width, area.height
            )?;
        } else {
            write!(writer, "\x1b_Gm={more};")?;
        }
        writer.write_all(chunk)?;
        writer.write_all(b"\x1b\\")?;
    }
    writer.flush()?;
    Ok(())
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let a = chunk[0];
        let b = chunk.get(1).copied().unwrap_or(0);
        let c = chunk.get(2).copied().unwrap_or(0);
        output.push(TABLE[(a >> 2) as usize] as char);
        output.push(TABLE[(((a & 0x03) << 4) | (b >> 4)) as usize] as char);
        output.push(if chunk.len() > 1 {
            TABLE[(((b & 0x0f) << 2) | (c >> 6)) as usize] as char
        } else {
            '='
        });
        output.push(if chunk.len() > 2 {
            TABLE[(c & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    output
}

fn portrait_bytes(name: &str) -> Option<&'static [u8]> {
    Some(match name {
        "Astraea, Starbound" => include_bytes!("../assets/characters/astraea.png"),
        "Kaelis, Ashen Vanguard" => include_bytes!("../assets/characters/kaelis.png"),
        "Seraphine, Verdant Oracle" => include_bytes!("../assets/characters/seraphine.png"),
        "Veyra, Stormseeker" => include_bytes!("../assets/characters/veyra.png"),
        "Orin, Keeper of Embers" => include_bytes!("../assets/characters/orin.png"),
        "Mira" => include_bytes!("../assets/characters/mira.png"),
        "Thorne" => include_bytes!("../assets/characters/thorne.png"),
        "Lumen" => include_bytes!("../assets/characters/lumen.png"),
        "Vaughn, Violet Oath" => include_bytes!("../assets/characters/vaughn.png"),
        "Steven, Azure Shade" => include_bytes!("../assets/characters/steven.png"),
        "Cinder, Forgeheart" => include_bytes!("../assets/characters/cinder.png"),
        "Sergei, Winterfang" => include_bytes!("../assets/characters/sergei.png"),
        "Zephra" => include_bytes!("../assets/characters/zephra.png"),
        "Neris" => include_bytes!("../assets/characters/neris.png"),
        "Brikka" => include_bytes!("../assets/characters/brikka.png"),
        "Saif, Dune Sovereign" => include_bytes!("../assets/characters/saif.png"),
        "Pyrite, Gilded Step" => include_bytes!("../assets/characters/pyrite.png"),
        "Jeanette, Tidemender" => include_bytes!("../assets/characters/jeanette.png"),
        "Polaris Edge" => include_bytes!("../assets/weapons/polaris_edge.png"),
        "Nova Grimoire" => include_bytes!("../assets/weapons/nova_grimoire.png"),
        "Celestial Atlas" => include_bytes!("../assets/weapons/celestial_atlas.png"),
        "Wolfsong Claymore" => include_bytes!("../assets/weapons/wolfsong_claymore.png"),
        "Moonlit Longbow" => include_bytes!("../assets/weapons/moonlit_longbow.png"),
        "Sage's Codex" => include_bytes!("../assets/weapons/sages_codex.png"),
        "Ironwind Blade" => include_bytes!("../assets/weapons/ironwind_blade.png"),
        "Galegrip Knuckles" => include_bytes!("../assets/weapons/galegrip_knuckles.png"),
        "Winter's Requiem" => include_bytes!("../assets/weapons/winters_requiem.png"),
        "Twin Cinderfangs" => include_bytes!("../assets/weapons/twin_cinderfangs.png"),
        "Duskward Spear" => include_bytes!("../assets/weapons/duskward_spear.png"),
        "Bellflower Greatsword" => include_bytes!("../assets/weapons/bellflower_greatsword.png"),
        "Farah" => include_bytes!("../assets/characters/farah.png"),
        "Anya" => include_bytes!("../assets/characters/anya.png"),
        "Rook" => include_bytes!("../assets/characters/rook.png"),
        "Kestrel" => include_bytes!("../assets/characters/kestrel.png"),
        "Mako" => include_bytes!("../assets/characters/mako.png"),
        "Ysra" => include_bytes!("../assets/characters/ysra.png"),
        "Dolma" => include_bytes!("../assets/characters/dolma.png"),
        "Corvin" => include_bytes!("../assets/characters/corvin.png"),
        "Dreamwood Recurve" => include_bytes!("../assets/weapons/dreamwood_recurve.png"),
        "Oathbreaker Thunder" => include_bytes!("../assets/weapons/oathbreaker_thunder.png"),
        "Veilfire Sutra" => include_bytes!("../assets/weapons/veilfire_sutra.png"),
        "White Hunt Reliquary" => include_bytes!("../assets/weapons/white_hunt_reliquary.png"),
        "Sandsworn Dominion" => include_bytes!("../assets/weapons/sandsworn_dominion.png"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_raster_item_has_protocol_portrait_bytes() {
        for name in [
            "Astraea, Starbound",
            "Kaelis, Ashen Vanguard",
            "Seraphine, Verdant Oracle",
            "Veyra, Stormseeker",
            "Orin, Keeper of Embers",
            "Mira",
            "Thorne",
            "Lumen",
            "Vaughn, Violet Oath",
            "Steven, Azure Shade",
            "Cinder, Forgeheart",
            "Sergei, Winterfang",
            "Zephra",
            "Neris",
            "Brikka",
            "Saif, Dune Sovereign",
            "Pyrite, Gilded Step",
            "Jeanette, Tidemender",
            "Polaris Edge",
            "Nova Grimoire",
            "Celestial Atlas",
            "Wolfsong Claymore",
            "Moonlit Longbow",
            "Sage's Codex",
            "Ironwind Blade",
            "Galegrip Knuckles",
            "Winter's Requiem",
            "Twin Cinderfangs",
            "Duskward Spear",
            "Bellflower Greatsword",
            "Farah",
            "Anya",
            "Rook",
            "Kestrel",
            "Mako",
            "Ysra",
            "Dolma",
            "Corvin",
            "Dreamwood Recurve",
            "Oathbreaker Thunder",
            "Veilfire Sutra",
            "White Hunt Reliquary",
            "Sandsworn Dominion",
        ] {
            assert!(
                portrait_bytes(name).is_some(),
                "missing protocol artwork for {name}"
            );
        }
    }

    #[test]
    fn base64_encoder_handles_padding() {
        assert_eq!(base64_encode(b"WishSim"), "V2lzaFNpbQ==");
        assert_eq!(base64_encode(b"art"), "YXJ0");
    }

    #[test]
    fn graphics_command_places_png_without_moving_the_cursor() {
        let mut output = Vec::new();
        display_png(&mut output, b"PNG", Rect::new(4, 6, 20, 12)).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.starts_with("\x1b[7;5H\x1b_Ga=T,f=100,t=d,i=9173,c=20,r=12,C=1,q=1,m=0;"));
        assert!(text.ends_with("UE5H\x1b\\"));
    }
}
