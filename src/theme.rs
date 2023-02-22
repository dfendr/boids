#[derive(PartialEq)]
pub enum Theme {
    Normal,
    Grey,
    DeepSea,
}

impl Theme {
    pub fn next(&self) -> Theme {
        match self {
            Theme::Normal => Theme::Grey,
            Theme::Grey => Theme::DeepSea,
            Theme::DeepSea => Theme::Normal,
        }
    }
}
