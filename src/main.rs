#[macro_use]
extern crate log;

mod app;
mod cli;
mod graph;
mod nature;
mod projector;
mod raw;
mod rect;
mod scale;
mod seq;
mod skin;
mod tbl;
mod unoverlap;

pub use {
    app::*,
    graph::*,
    nature::*,
    projector::*,
    raw::*,
    rect::*,
    scale::*,
    seq::*,
    skin::*,
    tbl::*,
    unoverlap::*,
};

fn main() -> anyhow::Result<()> {
    cli_log::init("csv2svg");
    cli::run()?;
    info!("bye");
    Ok(())
}
