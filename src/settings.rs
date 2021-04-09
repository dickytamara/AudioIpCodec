

use gtk::prelude::*;
use gtk::{Builder, Notebook};
use std::cell::RefCell;


#[derive(Clone)]
pub struct SettingsWidgetStorage {
    // gtk notebook
    ntbk_settings: Notebook,
}

impl SettingsWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsWidgetStorage {
            ntbk_settings: gtk_builder.get_object("ntbk_settings").unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct SettingsWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsWidgetStorage>
}


impl SettingsWidget {
    pub fn new(gtk_builder: &gtk::Builder) -> Self {
        SettingsWidget {
            ctx: RefCell::new(SettingsWidgetStorage::new(gtk_builder)),
        }
    }
}