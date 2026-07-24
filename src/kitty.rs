use std::io::{self, Write};

use color_eyre::eyre::Result;
use ratatui::layout::Rect;

const IMAGE_ID: u32 = 9_173;
const CHUNK_SIZE: usize = 4_096;

#[derive(Default)]
pub struct GraphicsRenderer {
    current: Vec<(String, Rect)>,
}

impl GraphicsRenderer {
    pub fn sync(&mut self, portraits: &[(&str, Rect)]) -> Result<()> {
        let next = portraits
            .iter()
            .map(|(name, area)| ((*name).to_owned(), *area))
            .collect::<Vec<_>>();
        if self.current == next {
            return Ok(());
        }
        self.clear()?;
        for (index, (name, area)) in next.iter().enumerate() {
            if let Some(bytes) = portrait_bytes(name) {
                display_png(&mut io::stdout(), bytes, *area, IMAGE_ID + index as u32)?;
            }
        }
        self.current = next;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        if !self.current.is_empty() {
            let mut stdout = io::stdout();
            for index in 0..self.current.len() {
                write!(
                    stdout,
                    "\x1b_Ga=d,d=I,i={},q=1\x1b\\",
                    IMAGE_ID + index as u32
                )?;
            }
            stdout.flush()?;
            self.current.clear();
        }
        Ok(())
    }
}

fn display_png(writer: &mut impl Write, png: &[u8], area: Rect, image_id: u32) -> Result<()> {
    let payload = base64_encode(png);
    write!(writer, "\x1b[{};{}H", area.y + 1, area.x + 1)?;
    for (index, chunk) in payload.as_bytes().chunks(CHUNK_SIZE).enumerate() {
        let more = usize::from((index + 1) * CHUNK_SIZE < payload.len());
        if index == 0 {
            write!(
                writer,
                "\x1b_Ga=T,f=100,t=d,i={image_id},c={},r={},C=1,q=1,m={more};",
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
    if let Some(name) = name.strip_prefix("face:") {
        return face_portrait_bytes(name);
    }
    Some(match name {
        "Hydro Slime" => include_bytes!("../assets/enemies/hydro_slime.png"),
        "Thornbloom" => include_bytes!("../assets/enemies/thornbloom.png"),
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
        "Yeoungin, Winter's Grace" => include_bytes!("../assets/characters/yeoungin.png"),
        "Klara, Jade Tempest" => include_bytes!("../assets/characters/klara.png"),
        "Pyrite, Gilded Step" => include_bytes!("../assets/characters/pyrite.png"),
        "Jeanette, Tidemender" => include_bytes!("../assets/characters/jeanette.png"),
        "Astral Ruin Knight" => include_bytes!("../assets/enemies/astral_ruin_knight.png"),
        "Ember Wisp" => include_bytes!("../assets/enemies/ember_wisp.png"),
        "Somnial Frostwyrm" => include_bytes!("../assets/enemies/somnial_frostwyrm.png"),
        "Mad Goliath" => include_bytes!("../assets/enemies/mad_goliath.png"),
        "Goliath Shardling" => include_bytes!("../assets/enemies/goliath_shardling.png"),
        "Polaris Edge" => include_bytes!("../assets/weapons/polaris_edge.png"),
        "Nova Grimoire" => include_bytes!("../assets/weapons/nova_grimoire.png"),
        "Celestial Atlas" => include_bytes!("../assets/weapons/celestial_atlas.png"),
        "Wolfsong Claymore" => include_bytes!("../assets/weapons/wolfsong_claymore.png"),
        "Tempest Meridian" => include_bytes!("../assets/weapons/tempest_meridian.png"),
        "Emberkeeper's Oath" => include_bytes!("../assets/weapons/emberkeepers_oath.png"),
        "Furnaceheart Bracers" => include_bytes!("../assets/weapons/furnaceheart_bracers.png"),
        "Aurum Flash" => include_bytes!("../assets/weapons/aurum_flash.png"),
        "Silver Tidemark" => include_bytes!("../assets/weapons/silver_tidemark.png"),
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
        "Seo-yeon" => include_bytes!("../assets/characters/seo-yeon.png"),
        "Ji-ho" => include_bytes!("../assets/characters/ji-ho.png"),
        "Taisia" => include_bytes!("../assets/characters/taisia.png"),
        "Dreamwood Recurve" => include_bytes!("../assets/weapons/dreamwood_recurve.png"),
        "Oathbreaker Thunder" => include_bytes!("../assets/weapons/oathbreaker_thunder.png"),
        "Veilfire Sutra" => include_bytes!("../assets/weapons/veilfire_sutra.png"),
        "White Hunt Reliquary" => include_bytes!("../assets/weapons/white_hunt_reliquary.png"),
        "Sandsworn Dominion" => include_bytes!("../assets/weapons/sandsworn_dominion.png"),
        "Rimebound Benediction" => {
            include_bytes!("../assets/weapons/rimebound_benediction.png")
        }
        "Gale's Last Harvest" => include_bytes!("../assets/weapons/gales_last_harvest.png"),
        "Dawncool Steel" => include_bytes!("../assets/weapons/dawncool_steel.png"),
        "Raven Bow" => include_bytes!("../assets/weapons/raven_bow.png"),
        "Quartz Spear" => include_bytes!("../assets/weapons/quartz_spear.png"),
        "Wanderer's Notes" => include_bytes!("../assets/weapons/wanderers_notes.png"),
        "Old Mercenary's Greatsword" => {
            include_bytes!("../assets/weapons/old_mercenarys_greatsword.png")
        }
        _ => return None,
    })
}

pub fn face_portrait_key(name: &str) -> Option<&'static str> {
    Some(match name {
        "Anya" => "face:Anya",
        "Astraea, Starbound" => "face:Astraea, Starbound",
        "Brikka" => "face:Brikka",
        "Cinder, Forgeheart" => "face:Cinder, Forgeheart",
        "Corvin" => "face:Corvin",
        "Dolma" => "face:Dolma",
        "Farah" => "face:Farah",
        "Jeanette, Tidemender" => "face:Jeanette, Tidemender",
        "Ji-ho" => "face:Ji-ho",
        "Kaelis, Ashen Vanguard" => "face:Kaelis, Ashen Vanguard",
        "Kestrel" => "face:Kestrel",
        "Lumen" => "face:Lumen",
        "Mako" => "face:Mako",
        "Mira" => "face:Mira",
        "Neris" => "face:Neris",
        "Orin, Keeper of Embers" => "face:Orin, Keeper of Embers",
        "Pyrite, Gilded Step" => "face:Pyrite, Gilded Step",
        "Rook" => "face:Rook",
        "Saif, Dune Sovereign" => "face:Saif, Dune Sovereign",
        "Seo-yeon" => "face:Seo-yeon",
        "Seraphine, Verdant Oracle" => "face:Seraphine, Verdant Oracle",
        "Sergei, Winterfang" => "face:Sergei, Winterfang",
        "Steven, Azure Shade" => "face:Steven, Azure Shade",
        "Thorne" => "face:Thorne",
        "Vaughn, Violet Oath" => "face:Vaughn, Violet Oath",
        "Veyra, Stormseeker" => "face:Veyra, Stormseeker",
        "Yeoungin, Winter's Grace" => "face:Yeoungin, Winter's Grace",
        "Klara, Jade Tempest" => "face:Klara, Jade Tempest",
        "Taisia" => "face:Taisia",
        "Ysra" => "face:Ysra",
        "Zephra" => "face:Zephra",
        _ => return None,
    })
}

fn face_portrait_bytes(name: &str) -> Option<&'static [u8]> {
    Some(match name {
        "Anya" => include_bytes!("../assets/portraits/anya.png"),
        "Astraea, Starbound" => include_bytes!("../assets/portraits/astraea.png"),
        "Brikka" => include_bytes!("../assets/portraits/brikka.png"),
        "Cinder, Forgeheart" => include_bytes!("../assets/portraits/cinder.png"),
        "Corvin" => include_bytes!("../assets/portraits/corvin.png"),
        "Dolma" => include_bytes!("../assets/portraits/dolma.png"),
        "Farah" => include_bytes!("../assets/portraits/farah.png"),
        "Jeanette, Tidemender" => include_bytes!("../assets/portraits/jeanette.png"),
        "Ji-ho" => include_bytes!("../assets/portraits/ji-ho.png"),
        "Kaelis, Ashen Vanguard" => include_bytes!("../assets/portraits/kaelis.png"),
        "Kestrel" => include_bytes!("../assets/portraits/kestrel.png"),
        "Lumen" => include_bytes!("../assets/portraits/lumen.png"),
        "Mako" => include_bytes!("../assets/portraits/mako.png"),
        "Mira" => include_bytes!("../assets/portraits/mira.png"),
        "Neris" => include_bytes!("../assets/portraits/neris.png"),
        "Orin, Keeper of Embers" => include_bytes!("../assets/portraits/orin.png"),
        "Pyrite, Gilded Step" => include_bytes!("../assets/portraits/pyrite.png"),
        "Rook" => include_bytes!("../assets/portraits/rook.png"),
        "Saif, Dune Sovereign" => include_bytes!("../assets/portraits/saif.png"),
        "Seo-yeon" => include_bytes!("../assets/portraits/seo-yeon.png"),
        "Seraphine, Verdant Oracle" => include_bytes!("../assets/portraits/seraphine.png"),
        "Sergei, Winterfang" => include_bytes!("../assets/portraits/sergei.png"),
        "Steven, Azure Shade" => include_bytes!("../assets/portraits/steven.png"),
        "Thorne" => include_bytes!("../assets/portraits/thorne.png"),
        "Vaughn, Violet Oath" => include_bytes!("../assets/portraits/vaughn.png"),
        "Veyra, Stormseeker" => include_bytes!("../assets/portraits/veyra.png"),
        "Yeoungin, Winter's Grace" => include_bytes!("../assets/portraits/yeoungin.png"),
        "Klara, Jade Tempest" => include_bytes!("../assets/portraits/klara.png"),
        "Taisia" => include_bytes!("../assets/portraits/taisia.png"),
        "Ysra" => include_bytes!("../assets/portraits/ysra.png"),
        "Zephra" => include_bytes!("../assets/portraits/zephra.png"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_raster_item_has_protocol_portrait_bytes() {
        for name in [
            "Hydro Slime",
            "Thornbloom",
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
            "Yeoungin, Winter's Grace",
            "Klara, Jade Tempest",
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
            "Seo-yeon",
            "Ji-ho",
            "Taisia",
            "Dreamwood Recurve",
            "Oathbreaker Thunder",
            "Veilfire Sutra",
            "White Hunt Reliquary",
            "Sandsworn Dominion",
            "Rimebound Benediction",
            "Gale's Last Harvest",
            "Dawncool Steel",
            "Raven Bow",
            "Quartz Spear",
            "Wanderer's Notes",
            "Old Mercenary's Greatsword",
        ] {
            assert!(
                portrait_bytes(name).is_some(),
                "missing protocol artwork for {name}"
            );
        }
    }

    #[test]
    fn every_character_face_has_protocol_portrait_bytes() {
        for character in crate::simulation::all_characters() {
            let key = face_portrait_key(character.name)
                .unwrap_or_else(|| panic!("missing face key for {}", character.name));
            assert!(
                portrait_bytes(key).is_some(),
                "missing face portrait bytes for {}",
                character.name
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
        display_png(&mut output, b"PNG", Rect::new(4, 6, 20, 12), IMAGE_ID).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.starts_with("\x1b[7;5H\x1b_Ga=T,f=100,t=d,i=9173,c=20,r=12,C=1,q=1,m=0;"));
        assert!(text.ends_with("UE5H\x1b\\"));
    }
}
