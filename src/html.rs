use {
    anyhow::*,
    std::{
        io::self,
        path::PathBuf,
    },
    svg::{
        Document,
    },
};

static CSS: &str = r#"
html, body { margin:0; padding:0; overflow:hidden; }
body { background:#222; }
svg { position:absolute; top:5%; left:5%; width:90%; height:90%; }
"#;

pub fn write_embedded<W: io::Write>(mut w: W, svg: &Document) -> Result<()> {
    writeln!(w, "<!DOCTYPE HTML>")?;
    writeln!(w, "<html>")?;
    writeln!(w, "<body>")?;
    writeln!(w, "<head>")?;
    writeln!(w, "<style type=text/css>{}</style>", CSS)?;
    writeln!(w, "</head>")?;
    svg::write(&mut w, svg)?;
    writeln!(w, "</body>")?;
    writeln!(w, "</html>")?;
    Ok(())
}

pub fn write_in_temp_file(svg: &Document) -> Result<PathBuf> {
    let (w, path) = tempfile::Builder::new()
        .prefix("csv2svg-")
        .suffix(".html")
        .rand_bytes(12)
        .tempfile()?
        .keep()
        .map_err(|_| io::Error::new(
            io::ErrorKind::Other,
            "temp file can't be kept",
        ))?;
    write_embedded(w, svg)?;
    Ok(path)
}


