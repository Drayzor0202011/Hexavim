use ratatui::{
    widgets::Paragraph,
    layout::Rect,
    text::{Spans, Span},
    style::{Style, Color},
};

use crate::buffer::Buffer;
use crate::editor::cursor::Cursor;

pub fn render(buffer: &Buffer, area: Rect, cursor: &Cursor) -> Paragraph {
    let mut lines = Vec::new();
    let bytes_per_line = 16;

    let start = cursor.scroll;
    let end = start + area.height as usize;

    for (i, chunk) in buffer.data.chunks(bytes_per_line).enumerate().skip(start).take(area.height as usize) {
        let offset = i * bytes_per_line;
        let mut spans = vec![Span::raw(format!("{:08X}: ", offset))];

        for (j, b) in chunk.iter().enumerate() {
            if i == cursor.y + cursor.scroll && j == cursor.x {
                spans.push(Span::styled(
                    format!("{:02X} ", b),
                    Style::default().bg(Color::Blue)
                ));
            } else {
                spans.push(Span::raw(format!("{:02X} ", b)));
            }
        }

        let ascii: String = chunk.iter()
            .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
            .collect();

        spans.push(Span::raw(format!(" {}", ascii)));
        lines.push(Spans::from(spans));
    }

    Paragraph::new(lines)
}
