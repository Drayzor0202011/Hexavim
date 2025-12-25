#[cfg(not(unix))]
compile_error!("hexvim só funciona em Linux/Unix. Windows e Android não são suportados.");
