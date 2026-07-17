mod app;
mod model;
mod simulation;
mod storage;
mod ui;

use std::io::{self, IsTerminal};

use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::Result;

use crate::{
    model::{Banner, SaveData},
    simulation::WishEngine,
};

#[derive(Clone, Copy, ValueEnum)]
enum BannerArg {
    Astraea,
    Kaelis,
    Seraphine,
    Weapon,
}

impl From<BannerArg> for Banner {
    fn from(value: BannerArg) -> Self {
        match value {
            BannerArg::Astraea => Self::Astraea,
            BannerArg::Kaelis => Self::Kaelis,
            BannerArg::Seraphine => Self::Seraphine,
            BannerArg::Weapon => Self::Weapon,
        }
    }
}

#[derive(Parser)]
#[command(
    name = "wishsim",
    version,
    about = "A cinematic terminal wish simulator"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Pull without opening the interactive interface
    Pull {
        #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=10))]
        count: u8,
        /// Reproduce a pull exactly (does not save)
        #[arg(long)]
        seed: Option<u64>,
        /// Banner to wish on
        #[arg(short, long, value_enum, default_value = "astraea")]
        banner: BannerArg,
    },
    /// Show saved pity, inventory, and pull totals
    Stats,
    /// Erase local simulator progress
    Reset,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Pull {
            count,
            seed,
            banner,
        }) => pull_plain(count, seed, banner.into()),
        Some(Command::Stats) => show_stats(),
        Some(Command::Reset) => {
            storage::reset()?;
            println!("Wish history and pity have been reset.");
            Ok(())
        }
        None if !io::stdout().is_terminal() => pull_plain(1, None, Banner::Astraea),
        None => app::run(),
    }
}

fn pull_plain(count: u8, seed: Option<u64>, banner: Banner) -> Result<()> {
    let mut save = storage::load()?;
    let mut engine = seed.map_or_else(WishEngine::random, WishEngine::seeded);
    let results = engine.pull_many(&mut save, count as usize, banner);
    for result in &results {
        println!(
            "{} {}{}",
            result.rarity.stars(),
            result.item.name,
            if result.featured { "  [featured]" } else { "" }
        );
    }
    if banner == Banner::Weapon {
        println!(
            "Weapon pity: {}/80  |  Fate: {}/1  |  Path: {}",
            save.weapon_pity.five_star,
            save.weapon_pity.fate_points,
            save.weapon_pity.path.name()
        );
    } else {
        println!(
            "5-star pity: {}  |  4-star pity: {}",
            save.pity.five_star, save.pity.four_star
        );
    }
    if seed.is_none() {
        storage::save(&save)?;
    }
    Ok(())
}

fn show_stats() -> Result<()> {
    let save: SaveData = storage::load()?;
    println!("Total wishes: {}", save.total_wishes);
    println!(
        "5-star pity: {}/90{}",
        save.pity.five_star,
        if save.pity.guaranteed_five {
            " (featured guaranteed)"
        } else {
            ""
        }
    );
    println!(
        "4-star pity: {}/10{}",
        save.pity.four_star,
        if save.pity.guaranteed_four {
            " (featured guaranteed)"
        } else {
            ""
        }
    );
    println!("Unique items: {}", save.inventory.len());
    println!("Weapon pity: {}/80", save.weapon_pity.five_star);
    println!(
        "Epitomized path: {} ({}/1 Fate)",
        save.weapon_pity.path.name(),
        save.weapon_pity.fate_points
    );
    Ok(())
}
