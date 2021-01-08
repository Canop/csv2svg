mod args;

pub use args::*;

use {
    crate::*,
    anyhow::*,
    argh,
    std::io::{self, Write},
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
    let svg = graph.build_svg();
    if is_output_piped() {
        // if the output is piped, we produce the requested
        // SVG (which may end in a file for example)
        let mut w = io::stdout();
        debug!("writing svg to stdout");
        svg::write(&mut w, &svg)?;
        w.write_all(b"\n")?;
    } else {
        // output isn't piped, we open the graph in the browser
        let path = html::write_in_temp_file(&svg)?;
        debug!("wrote html in temp file {:?}", &path);
        open::that(path)?;
    }
    Ok(())
}

fn is_output_piped() -> bool {
    unsafe {
        libc::isatty(libc::STDOUT_FILENO) == 0
    }
}
