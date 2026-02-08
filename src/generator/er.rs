use std::io::{self, Write};

use rand::Rng;

pub fn generate_erdos_renyi<R: Rng, W: Write>(
    nodes: u32,
    edges: u64,
    rng: &mut R,
    mut writer: W,
) -> io::Result<()> {
    for _ in 0..edges {
        let src = rng.gen_range(0..nodes);
        let dst = rng.gen_range(0..nodes);
        writeln!(writer, "{} {}", src, dst)?;
    }
    Ok(())
}
