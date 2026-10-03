#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Layer {
    Background,
    BackgroundUi,
    Entity,
    ForegroundUi,
}