pub struct Cursor {
    pub x: usize,      // coluna (0-15)
    pub y: usize,      // linha visível
    pub scroll: usize // linha inicial
}

impl Cursor {
    pub fn new() -> Self {
        Self { x: 0, y: 0, scroll: 0 }
    }

    pub fn left(&mut self) { if self.x > 0 { self.x -= 1; } }
    pub fn right(&mut self) { if self.x < 15 { self.x += 1; } }
    pub fn up(&mut self) {
        if self.y > 0 { self.y -= 1; }
        else if self.scroll > 0 { self.scroll -= 1; }
    }
    pub fn down(&mut self, max_lines: usize, height: usize) {
        if self.y + self.scroll + 1 < max_lines {
            if self.y + 1 < height { self.y += 1; }
            else { self.scroll += 1; }
        }
    }
}
