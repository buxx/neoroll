use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use neoroll_world::generator::WorldGenerator;
use neoroll_world::{
    generator::perlin_noise_simple::PerlinNoiseSimpleGenerator, map::builder::MapBuilder,
};
use rand::{distributions::Alphanumeric, Rng};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(long = "--seed")]
    seed: Option<String>,

    #[structopt(short, long)]
    lakes: bool,

    #[structopt()]
    lines: i64,

    #[structopt()]
    columns: i64,

    #[structopt()]
    nm1_from: f64,

    #[structopt()]
    nm1_to: f64,

    #[structopt()]
    nm2_from: f64,

    #[structopt()]
    nm2_to: f64,

    #[structopt()]
    nm2_factor: i64,

    #[structopt(parse(from_os_str))]
    world_output: PathBuf,

    #[structopt(parse(from_os_str))]
    map_output: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let seed = &opt.seed.unwrap_or(
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect(),
    );

    let world = PerlinNoiseSimpleGenerator::new(
        seed,
        opt.lines,
        opt.columns,
        opt.nm1_from,
        opt.nm1_to,
        opt.nm2_from,
        opt.nm2_to,
        opt.nm2_factor,
    )
    .generate();
    let map = MapBuilder::new(&world).build_lakes(opt.lakes).build();

    let world_output_display = opt.world_output.display().to_string();
    let map_output_display = opt.map_output.display().to_string();
    fs::write(opt.world_output, bincode::serialize(&world)?)
        .context(format!("Write world into file '{}'", &world_output_display))?;
    fs::write(opt.map_output, bincode::serialize(&map)?)
        .context(format!("Write map into file '{}'", &map_output_display))?;

    Ok(())
}
