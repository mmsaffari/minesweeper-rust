#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardProperties {
    pub rows: usize,
    pub cols: usize,
    pub mines: usize,
}
