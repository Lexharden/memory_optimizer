use std::fs;
use std::io;
use crate::utils::get_cache_dir;

pub fn clear_cache() -> io::Result<()> {
    if let Some(cache_dir) = get_cache_dir() {
        for entry in fs::read_dir(cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                match fs::remove_file(&path) {
                    Ok(_) => println!("Deleted file: {:?}", path),
                    Err(e) => eprintln!("Failed to delete file {:?}: {}", path, e),
                }
            } else if path.is_dir() {
                match fs::remove_dir_all(&path) {
                    Ok(_) => println!("Deleted directory: {:?}", path),
                    Err(e) => eprintln!("Failed to delete directory {:?}: {}", path, e),
                }
            }
        }
    }
    Ok(())
}
