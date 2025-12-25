use std::io;
use std::ops::Index;

pub struct MmapBuffer {
    data: Vec<u8>,
}

impl MmapBuffer {
    pub fn open(path: &str) -> io::Result<Self> {
        let data = std::fs::read(path)?;
        Ok(Self { data })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Index<usize> for MmapBuffer {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
