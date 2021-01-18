#[macro_use] extern crate log;

fn main() -> anyhow::Result<()> {
    cli_log::init("csv2svg");
    csv2svg::run()?;
    info!("bye");
    Ok(())
}
