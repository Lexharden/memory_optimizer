mod cleaner;
mod memory;
mod utils;

use std::{thread, time};
use chrono::Local;

fn main() {
    let interval = time::Duration::from_secs(3600); // 1 hour interval

    loop {
        let now = Local::now();
        println!("Starting maintenance at: {}", now);

        match cleaner::temp::clear_temp_files() {
            Ok(_) => println!("Temporary files cleared successfully."),
            Err(e) => eprintln!("Failed to clear temporary files: {}", e),
        }

        match cleaner::cache::clear_cache() {
            Ok(_) => println!("Cache cleared successfully."),
            Err(e) => eprintln!("Failed to clear cache: {}", e),
        }

        memory::optimizer::free_memory();
        println!("Memory has been optimized.");

        println!("Next maintenance at: {}", now + chrono::Duration::seconds(3600));
        thread::sleep(interval);
    }
}
