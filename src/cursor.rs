#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum CursorMode {
    Attract,
    Avoid,
    Ignore,
}
impl CursorMode {
    pub fn next(&self) -> CursorMode {
        match *self {
            CursorMode::Attract => CursorMode::Avoid,
            CursorMode::Avoid => CursorMode::Ignore,
            CursorMode::Ignore => CursorMode::Attract,
        }
    }
}
