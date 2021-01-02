mod args;

pub use args::*;

use {
    crate::*,
    anyhow::*,
    argh,
    std::{
        io,
    },
};

pub fn run() -> Result<()> {
    let args: Args = argh::from_env();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("csv2svg {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let raw_tbl = RawTbl::read(io::stdin())?;
    //debug!("raw tbl: {:#?}", &raw_tbl);
    let tbl = Tbl::new(raw_tbl)?;
    //debug!("tbl: {:#?}", &tbl);
    debug!("tbl dim: {:?}", tbl.dim());
    let graph = Graph::new(tbl);
    let mut w = io::stdout();
    graph.write_svg(&mut w)?;
    //writeln!(w, "");
    Ok(())
}
