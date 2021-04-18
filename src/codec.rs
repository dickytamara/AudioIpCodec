use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct CodecStorage {

}


impl CodecStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        CodecStorage{

        }
    }
}


#[derive(Clone)]
pub struct CodecWidget {
    ctx: RefCell<CodecStorage>
}


impl CodecWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = CodecWidget {
            ctx: RefCell::new(CodecStorage::new(gtk_builder)),
        };

        result
    }

    pub fn reset(&self) {
        todo!();
    }


}


impl HelperFileSettings for CodecWidget {
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();


        // config.write(path.to_str().unwrap()).unwrap();
    }
}