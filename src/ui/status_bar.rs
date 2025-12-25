use ratatui::{
    widgets::{Paragraph, Block, Borders},
    text::Line,
};

pub fn render() -> Paragraph<'static> {
    Paragraph::new(Line::from(" NORMAL  |  offset 0x00000000  |  0% "))
        .block(Block::default().borders(Borders::TOP))
}
