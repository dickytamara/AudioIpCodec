
use gtk::prelude::*;
use gtk::{Switch, Label, Builder};
use std::cell::RefCell;
use std::path::PathBuf;


use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsUaWidgetStorage {
    lbl_autoanswer: Label,
    lbl_no_refersub: Label,
    lbl_compact_form: Label,
    lbl_no_forcelr: Label,
    swt_autoanswer: Switch,
    swt_no_refersub: Switch,
    swt_compact_form: Switch,
    swt_no_forcelr: Switch
}

impl SettingsUaWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsUaWidgetStorage {
            lbl_autoanswer: gtk_builder.get_object("lbl_ua_autoanswer").unwrap(),
            lbl_no_refersub: gtk_builder.get_object("lbl_ua_no_refersub").unwrap(),
            lbl_compact_form: gtk_builder.get_object("lbl_ua_compact_form").unwrap(),
            lbl_no_forcelr: gtk_builder.get_object("lbl_ua_no_forcelr").unwrap(),
            swt_autoanswer: gtk_builder.get_object("swt_ua_autoanswer").unwrap(),
            swt_no_refersub: gtk_builder.get_object("swt_ua_no_refersub").unwrap(),
            swt_compact_form: gtk_builder.get_object("swt_ua_compact_form").unwrap(),
            swt_no_forcelr: gtk_builder.get_object("swt_ua_no_forcelr").unwrap()
        }
    }
}

#[derive(Clone)]
pub struct SettingsUaWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsUaWidgetStorage>
}

impl SettingsUaWidget {

    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsUaWidget{
            ctx: RefCell::new(SettingsUaWidgetStorage::new(gtk_builder))
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


impl HelperFileSettings for SettingsUaWidget {

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




