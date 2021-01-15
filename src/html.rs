use {
    anyhow::*,
    std::{
        io::self,
    },
    svg::{
        Document,
    },
};

static CSS: &str = r#"
html, body { margin:0; padding:0; overflow:hidden; }
body { background:#222; }
svg { position:absolute; top:5%; left:5%; width:90%; height:90%; }
svg g.fad g.opt { opacity:.5; }
svg g.fad:hover g.opt { opacity:1; }
svg g.inv g.opt { display:none; }
svg g.inv:hover g.opt { display:block; }
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


