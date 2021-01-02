#[macro_use]
extern crate log;

mod app;
mod cli;
mod skin;
mod graph;
mod nature;
mod projector;
mod scale;
mod seq;
mod raw;
mod tbl;

pub use {
    app::*,
    skin::*,
    graph::*,
    nature::*,
    projector::*,
    scale::*,
    seq::*,
    raw::*,
    tbl::*,
};

fn main() -> anyhow::Result<()> {
    cli_log::init("csv2svg");
    cli::run()?;
    info!("bye");
    Ok(())
}
