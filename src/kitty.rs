use std::{
    io::Write,
    process::{Command, Stdio},
};

use color_eyre::eyre::Result;
use ratatui::layout::Rect;

#[derive(Default)]
pub struct KittyRenderer {
    current: Option<(String, Rect)>,
}

impl KittyRenderer {
    pub fn sync(&mut self, portrait: Option<(&str, Rect)>) -> Result<()> {
        let next = portrait.map(|(name, area)| (name.to_owned(), area));
        if self.current == next {
            return Ok(());
        }
        self.clear()?;
        if let Some((name, area)) = &next
            && let Some(bytes) = portrait_bytes(name)
        {
            let place = format!("{}x{}@{}x{}", area.width, area.height, area.x, area.y);
            if let Ok(mut child) = Command::new("kitten")
                .args([
                    "icat",
                    "--stdin=yes",
                    "--transfer-mode=stream",
                    "--scale-up=yes",
                    "--align=center",
                    "--no-trailing-newline",
                    "--place",
                    &place,
                ])
                .stdin(Stdio::piped())
                .stdout(Stdio::inherit())
                .stderr(Stdio::null())
                .spawn()
            {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(bytes);
                }
                let _ = child.wait();
            }
        }
        self.current = next;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        if self.current.is_some() {
            let _ = Command::new("kitten")
                .args(["icat", "--stdin=no", "--clear"])
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::null())
                .status();
            self.current = None;
        }
        Ok(())
    }
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
        _ => return None,
    })
}
