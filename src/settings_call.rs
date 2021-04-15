
use gtk::prelude::*;
use gtk::{Switch, Label, Builder};
use std::cell::RefCell;
use std::path::PathBuf;


use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsCallWidgetStorage {
    swt_autoanswer: Switch,
    lbl_autoanswer: Label,
}

impl SettingsCallWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsCallWidgetStorage {
            swt_autoanswer: gtk_builder.get_object("swt_autoanswer").unwrap(),
            lbl_autoanswer: gtk_builder.get_object("lbl_autoanswer").unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct SettingsCallWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsCallWidgetStorage>
}

impl SettingsCallWidget {

    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsCallWidget{
            ctx: RefCell::new(SettingsCallWidgetStorage::new(gtk_builder))
        }
    }

    /// sub procedure reset state gui
    pub fn reset(&self) {
        self.ctx.borrow().swt_autoanswer.set_state(false);
    }

    // set autoanswer state
    pub fn set_autoanswer(&self, value: bool) {
        self.ctx.borrow().swt_autoanswer.set_state(value);
    }

    // get autoanswer state
    pub fn get_autoanswer(&self) -> bool {
        self.ctx.borrow().swt_autoanswer.get_state()
    }
}


impl HelperFileSettings for SettingsCallWidget {

    // load from file
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        self.set_autoanswer(config.get("call", "autoanswer").unwrap().parse().unwrap());
    }

    // save to file
    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        config.set("call", "autoanswer", Some(self.get_autoanswer().to_string()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}




