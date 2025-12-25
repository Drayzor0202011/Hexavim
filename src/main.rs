#[cfg(not(target_os = "linux"))]
compile_error!("Hexvim só funciona em Linux desktop.");

mod app;
mod buffer;
mod ui;

fn main() {
    
}
