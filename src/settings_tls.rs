use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;



pub struct SettingsTlsStorage {

}

impl SettingsTlsStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsTlsStorage {

        }
    }
}

pub struct SettingsTlsWidget {
    ctx: RefCell<SettingsTlsStorage>
}

impl SettingsTlsWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsTlsWidget {
            ctx: RefCell::new(SettingsTlsStorage::new(gtk_builder))
        };

        result
    }
}


impl HelperFileSettings for SettingsTlsWidget {
    fn load(&self, path: PathBuf) {
        todo!()
    }

    fn save(&self, path: PathBuf) {
        todo!()
    }
}


