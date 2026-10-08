use strum::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Display)]
pub enum Layer {
    Background,
    BackgroundUi,
    Entity,
    ForegroundUi,
}
