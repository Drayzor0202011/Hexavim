use ratatui::{
    widgets::{Paragraph, Block, Borders},
    text::{Line, Span},
};

pub fn render() -> Paragraph<'static> {
    let lines = vec![
        Line::from(vec![
            Span::raw("00000000 "),
            Span::raw("CA FE BA BE 00 00 00 02 "),
            Span::raw("...."),
        ]),
        Line::from(vec![
            Span::raw("00000010 "),
            Span::raw("48 65 78 76 69 6D 21 "),
            Span::raw("Hexvim!"),
        ]),
    ];

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE))
}
