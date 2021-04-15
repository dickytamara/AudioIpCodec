use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsMediaStorage {

}


impl SettingsMediaStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsMediaStorage {
            // TODO
        }
    }
}

#[derive(Clone)]
pub struct SettingsMediaWidget {
    ctx: RefCell<SettingsMediaStorage>
}


impl SettingsMediaWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsMediaWidget{
            ctx: RefCell::new(SettingsMediaStorage::new(gtk_builder))
        };

        result
    }

    pub fn reset(&self) {
        todo!();
    }
}


impl HelperFileSettings for SettingsMediaWidget {
    fn load(&self, path: PathBuf) {
        todo!()
    }

    fn save(&self, path: PathBuf) {
        todo!()
    }
}
