use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// I need to explain this, I guess
///
///
/// Source at https://github.com/Canop/csv2svg
pub struct Args {
    /// print the version
    #[argh(switch, short = 'v')]
    pub version: bool,

}

