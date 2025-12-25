use crate::buffer::mmap::MmapBuffer;

pub struct App {
    pub buffer: MmapBuffer,
    pub cursor: usize,
}

impl App {
    pub fn new(buffer: MmapBuffer) -> Self {
        Self {
            buffer,
            cursor: 0,
        }
    }
}
