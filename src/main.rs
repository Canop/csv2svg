#[macro_use]
extern crate log;

mod app;
mod cli;
mod graph;
mod html;
mod nature;
mod projector;
mod raw;
mod rect;
mod scale;
mod seq;
mod skin;
mod tbl;
mod unoverlap;
mod visibility;

pub use {
    app::*,
    graph::*,
    html::*,
    nature::*,
    projector::*,
    raw::*,
    rect::*,
    scale::*,
    seq::*,
    skin::*,
    tbl::*,
    unoverlap::*,
    visibility::*,
};

fn main() -> anyhow::Result<()> {
    cli_log::init("csv2svg");
    cli::run()?;
    info!("bye");
    Ok(())
}
