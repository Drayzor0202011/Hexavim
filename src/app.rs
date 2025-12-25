use crate::buffer::Buffer;
use crate::editor::modes::Mode;
use crate::editor::cursor::Cursor;

pub struct App {
    pub buffers: Vec<Buffer>,
    pub current_buffer: usize,
    pub cursor: Cursor,
    pub mode: Mode,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            current_buffer: 0,
            cursor: Cursor::new(),
            mode: Mode::Normal,
            should_quit: false,
        }
    }

    pub fn current_buffer(&self) -> Option<&Buffer> {
        self.buffers.get(self.current_buffer)
    }
}
