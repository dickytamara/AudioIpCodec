use gtk::prelude::*;
use gtk::{Label, ComboBoxText, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsMediaStorage {
    lbl_rtcp_multiplexing: Label,
    lbl_keyring: Label,
    swt_rtcp_multiplexing: Switch,
    cmb_keyring: ComboBoxText
}


impl SettingsMediaStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsMediaStorage {
            lbl_rtcp_multiplexing: gtk_builder.get_object("lbl_media_rtcp_multiplexing").unwrap(),
            lbl_keyring: gtk_builder.get_object("lbl_media_keyring").unwrap(),
            swt_rtcp_multiplexing: gtk_builder.get_object("swt_media_rtcp_multiplexing").unwrap(),
            cmb_keyring: gtk_builder.get_object("cmb_media_keyring").unwrap()
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
        self.ctx.borrow().swt_rtcp_multiplexing.set_state(false);
        self.ctx.borrow().cmb_keyring.set_active(Some(0));
    }

    pub fn set_rtcp_multiplexing(&self, value: bool) {
        self.ctx.borrow_mut().swt_rtcp_multiplexing.set_state(value);
    }

    pub fn get_rtcp_multiplexing(&self) -> bool {
        self.ctx.borrow_mut().swt_rtcp_multiplexing.get_state()
    }

    pub fn set_keyring(&self, value: u32) {
        self.ctx.borrow().cmb_keyring.set_active(Some(value -1));
    }

    pub fn get_keyring(&self) -> u32 {
        match self.ctx.borrow().cmb_keyring.get_active() {
            Some(value) => value +1,
            None => 0
        }
    }
}


impl HelperFileSettings for SettingsMediaWidget {
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        self.set_rtcp_multiplexing(config.get("media", "rtcp_multiplexing").unwrap_or(String::from("false")).parse().unwrap());
        self.set_keyring(config.get("media", "keyring").unwrap_or(String::from("0")).parse().unwrap());
    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        config.set("media", "rtcp_multiplexing", Some(self.get_rtcp_multiplexing().to_string()));
        config.set("media", "keyring", Some(self.get_keyring().to_string()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}
