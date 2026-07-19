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
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_character_has_protocol_portrait_bytes() {
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
        ] {
            assert!(
                portrait_bytes(name).is_some(),
                "missing Kitty portrait for {name}"
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
