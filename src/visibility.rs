#[derive(Debug, Clone, Copy)]
pub enum Visibility {
    Visible,
    Faded,
    Invisible,
}

impl Visibility {
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Visible => "std",
            Self::Faded => "fad",
            Self::Invisible => "inv",
        }
    }
}
