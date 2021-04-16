
use gtk::prelude::*;
use gtk::{Label, ComboBoxText, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsTurnWidgetStorage {
    lbl_use_turn: Label,
    lbl_transport: Label,
    lbl_server: Label,
    lbl_username: Label,
    lbl_password: Label,
    swt_use_turn: Switch,
    cmb_transport: ComboBoxText,
    ent_server: Entry,
    ent_username: Entry,
    ent_password: Entry,
}

impl SettingsTurnWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsTurnWidgetStorage {
            lbl_use_turn: gtk_builder.get_object("lbl_use_turn").unwrap(),
            lbl_transport: gtk_builder.get_object("lbl_turn_transport").unwrap(),
            lbl_server: gtk_builder.get_object("lbl_turn_server").unwrap(),
            lbl_username: gtk_builder.get_object("lbl_turn_username").unwrap(),
            lbl_password: gtk_builder.get_object("lbl_turn_password").unwrap(),
            swt_use_turn: gtk_builder.get_object("swt_use_turn").unwrap(),
            cmb_transport: gtk_builder.get_object("cmb_turn_transport").unwrap(),
            ent_server: gtk_builder.get_object("ent_turn_server").unwrap(),
            ent_username: gtk_builder.get_object("ent_turn_username").unwrap(),
            ent_password: gtk_builder.get_object("ent_turn_password").unwrap(),
        }
    }
}


#[derive(Clone)]
pub struct SettingsTurnWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsTurnWidgetStorage>
}

impl SettingsTurnWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsTurnWidget {
            ctx: RefCell::new(SettingsTurnWidgetStorage::new(gtk_builder)),
        };

        // set spin button turn port server
        let this = result.clone();
        result.ctx.borrow().swt_use_turn.connect_property_active_notify(move |s| {
            this.set_use_turn(s.get_state())
        });

        result.set_use_turn(false);
        result
    }

    pub fn reset(&self) {
        // set default value for turn properties
        let context = self.ctx.borrow();
        context.swt_use_turn.set_state(false);

        context.ent_server.set_text("");
        context.ent_username.set_text("");
        context.ent_password.set_text("");
        self.set_use_turn(false);
    }

    pub fn set_use_turn(&self, value: bool) {
        // label update
        self.ctx.borrow().lbl_transport.set_sensitive(value);
        self.ctx.borrow().lbl_server.set_sensitive(value);
        self.ctx.borrow().lbl_username.set_sensitive(value);
        self.ctx.borrow().lbl_password.set_sensitive(value);

        // input update
        self.ctx.borrow().cmb_transport.set_sensitive(value);
        self.ctx.borrow().ent_server.set_sensitive(value);
        self.ctx.borrow().ent_username.set_sensitive(value);
        self.ctx.borrow().ent_password.set_sensitive(value);
    }

    pub fn get_use_turn(&self) -> bool {
        self.ctx.borrow().swt_use_turn.get_state()
    }

    pub fn set_transport(&self, value: u32) {
        self.ctx.borrow_mut().cmb_transport.set_active(Some(value -1));
    }

    pub fn get_transport(&self) -> u32 {
        match self.ctx.borrow().cmb_transport.get_active() {
            Some(value) => value + 1,
            None => 0
        }
    }

    pub fn get_server(&self) -> String {
        self.ctx.borrow().ent_server.get_text().to_string().clone()
    }

    pub fn set_server(&self, value: String) {
        self.ctx.borrow().ent_server.set_text(value.as_str());
    }

    pub fn set_username(&self, value: String) {
        self.ctx.borrow().ent_username.set_text(value.as_str());
    }

    pub fn get_username(&self) -> String {
        self.ctx.borrow().ent_username.get_text().to_string().clone()
    }

    pub fn set_password(&self, value: String) {
        self.ctx.borrow().ent_password.set_text(value.as_str());
    }

    pub fn get_password(&self) -> String {
        self.ctx.borrow().ent_password.get_text().to_string().clone()
    }

}


impl HelperFileSettings for SettingsTurnWidget {

    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        self.set_use_turn (config.get("turn", "use_turn").unwrap_or(String::from("false")).parse().unwrap());
        self.set_transport(config.get("turn", "transport").unwrap_or(String::from("1")).parse().unwrap());
        self.set_server(config.get("turn", "server").unwrap());
        self.set_username(config.get("turn", "username").unwrap());
        self.set_password(config.get("turn", "password").unwrap());
    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        config.set("stun", "use_turn", Some(self.get_use_turn().to_string()));
        config.set("turn", "transport", Some(self.get_transport().to_string()));
        config.set("stun", "server", Some(self.get_server()));
        config.set("stun", "username", Some(self.get_username()));
        config.set("stun", "password", Some(self.get_password()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}


