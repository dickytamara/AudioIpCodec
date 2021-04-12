use std::path::PathBuf;
use std::fs::File;


/// trait that's implements load and save to ini files
pub trait HelperFileSettings {
    fn load(&self, path: PathBuf);
    fn save(&self, path: PathBuf);
}


pub fn application_config_path() -> PathBuf {

    let mut result = PathBuf::new();

    if cfg!(debug_assertions) {
        // if debugging config
        result = std::env::current_dir()
        .expect("Error::get_application_config()")
        .join("audioipcodec.ini");

        if !result.exists() {
            let new = File::create(result.as_path().clone())
            .expect("Error::application_config_path PERMISIONS.");
        }

    } else {
        // release in unix like operating system
        if cfg!(unix) {

        }
        // release in windows operating system
        else if cfg!(windows) {
            todo!();
        }
        // release in mac osx like operating system
        else if cfg!(macos) {
            todo!();
        }

    } 

    result
}