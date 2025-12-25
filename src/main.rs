mod app;
mod buffer;
mod editor;
mod ui;

use app::App;
use buffer::Buffer;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use std::io::{self, Write};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.buffers.push(Buffer::new("dummy", (0..=255).collect()));

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let buf = app.current_buffer().unwrap();
            let widget = ui::hex_view::render(buf, size, &app.cursor);
            f.render_widget(widget, size);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => app.cursor.left(),
                    KeyCode::Char('l') => app.cursor.right(),
                    KeyCode::Char('k') => app.cursor.up(),
                    KeyCode::Char('j') => {
                        let lines = app.current_buffer().unwrap().data.len() / 16;
                        app.cursor.down(lines, terminal.size()?.height as usize);
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
