// use gtk::prelude::*;
use gtk::{Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct AboutStorage {

}

impl AboutStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        AboutStorage {

        }
    }
}


#[derive(Clone)]
pub struct AboutWidget {
    ctx: RefCell<AboutStorage>
}

impl AboutWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = AboutWidget {
            ctx: RefCell::new(AboutStorage::new(gtk_builder))
        };


        result
    }

    pub fn reset(&self) {
        todo!();
    }
}


impl HelperFileSettings for AboutWidget {
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        // config.write(path.to_str().uwrap()).unwrap();
    }
}