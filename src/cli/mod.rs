mod args;

pub use args::*;

use {
    crate::*,
    anyhow::*,
    argh,
    std::{
        fs::File,
        io::{self, Write},
        path::PathBuf,
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
    let svg = graph.build_svg();
    if is_output_piped() {
        // when the output is piped, the default format is svg
        let mut w = io::stdout();
        match args.format {
            Some(Format::Html) => {
                html::write_embedded(&mut w, &svg)?;
            }
            _ => {
                svg::write(&mut w, &svg)?;
            }
        }
        w.write_all(b"\n")?;
    } else {
        // when the output isn't piped, we'll write a temp file
        // and ask the system to open it;
        // As it's the most expressive format, we prefer to
        // open some HTML in a browser
        let (mut w, path) = temp_file()?;
        match args.format {
            Some(Format::Svg) => {
                svg::write(&mut w, &svg)?;
            }
            _ => {
                html::write_embedded(&mut w, &svg)?;
            }
        }
        open::that(path)?;
    }
    Ok(())
}


fn is_output_piped() -> bool {
    unsafe {
        libc::isatty(libc::STDOUT_FILENO) == 0
    }
}

pub fn temp_file() -> io::Result<(File, PathBuf)> {
    tempfile::Builder::new()
        .prefix("csv2svg-")
        .suffix(".html")
        .rand_bytes(12)
        .tempfile()?
        .keep()
        .map_err(|_| io::Error::new(
            io::ErrorKind::Other,
            "temp file can't be kept",
        ))
}

