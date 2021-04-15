use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsBufferStorage {

}

impl SettingsBufferStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsBufferStorage {

        }
    }
}

#[derive(Clone)]
pub struct SettingsBufferWidget {
    ctx: RefCell<SettingsBufferStorage>
}

impl SettingsBufferWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsBufferWidget{
            ctx: RefCell::new(SettingsBufferStorage::new(gtk_builder)),
        };

        result
    }

    pub fn reset(&self) {
        todo!();
    }
}


impl HelperFileSettings for SettingsBufferWidget {
    fn load(&self, path: PathBuf) {
        todo!()
    }

    fn save(&self, path: PathBuf) {
        todo!()
    }
}

