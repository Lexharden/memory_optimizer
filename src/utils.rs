use std::env;
use std::path::PathBuf;

pub fn get_cache_dir() -> Option<PathBuf> {
    env::var_os("LOCALAPPDATA").map(PathBuf::from).map(|mut path| {
        path.push("Cache");
        path
    })
}

pub fn get_temp_dir() -> Option<PathBuf> {
    env::var_os("TEMP").map(PathBuf::from)
}