#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Moves {
    Rock,
    Paper,
    Scissors,
    NoMove, // immense looser
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    WinA,
    WinB,
    Draw,
}
