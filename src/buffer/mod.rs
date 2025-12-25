pub mod mmap;

pub struct Buffer {
    pub name: String,
    pub data: Vec<u8>,
}

impl Buffer {
    pub fn new(name: &str, data: Vec<u8>) -> Self {
        Self {
            name: name.to_string(),
            data,
        }
    }
}
