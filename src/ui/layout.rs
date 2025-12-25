use ratatui::layout::{Layout, Direction, Constraint, Rect};

pub struct HexLayout {
    pub main: Rect,
    pub status: Rect,
}

pub fn compute(area: Rect) -> HexLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    HexLayout {
        main: chunks[0],
        status: chunks[1],
    }
}
