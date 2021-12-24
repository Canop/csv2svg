#[macro_use] extern crate cli_log;

fn main() -> anyhow::Result<()> {
    init_cli_log!();
    csv2svg::run()?;
    info!("bye");
    Ok(())
}
