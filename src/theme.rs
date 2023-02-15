#[derive(PartialEq)]
pub enum Theme {
    Normal,
    DeepSea,
}

impl Theme {
    pub fn next(&self) -> Theme {
        match self {
            Theme::Normal => Theme::DeepSea,
            Theme::DeepSea => Theme::Normal,
        }
    }
}
