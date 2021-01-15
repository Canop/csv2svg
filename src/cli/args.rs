use {
    argh::FromArgs,
};

#[derive(Debug, FromArgs)]
/// I need to explain this, I guess
///
/// Source at https://github.com/Canop/csv2svg
pub struct Args {

    #[argh(switch, short = 'v')]
    /// print the version
    pub version: bool,


    #[argh(option, short='f')]
    /// output format: "svg" or "html"
    pub format: Option<Format>,
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Svg,
    Html,
}

impl std::str::FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_ref() {
            "s" | "svg" => Ok(Self::Svg),
            "h" | "html" => Ok(Self::Html),
            _ => Err(format!("unrecognized format {:?}", s))
        }
    }
}
