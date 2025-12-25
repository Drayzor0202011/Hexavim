#[cfg(not(target_os = "linux"))]
compile_error!("Hexvim só em Linux bb.");

mod app;
mod buffer;
mod ui;

fn main() {
    
}
