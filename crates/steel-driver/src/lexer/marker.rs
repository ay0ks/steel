#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Position(pub usize, pub usize);

impl From<(usize, usize)> for Position {
    fn from((line, column): (usize, usize)) -> Self {
        Self(line, column)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Marker {
    #[default]
    None,
    Begin(Position),
    End(Position),
    BeginBlock(Position),
    EndBlock(Position),
    BeginCharacter(Position),
    EndCharacter(Position),
    BeginString(Position),
    EndString(Position),
}
