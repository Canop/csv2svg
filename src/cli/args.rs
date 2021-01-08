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

}

