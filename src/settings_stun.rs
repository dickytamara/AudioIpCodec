

use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;

#[derive(Clone)]
pub struct SettingsStunStorage {
    lbl_server1: Label,
    lbl_server2: Label,
    lbl_server3: Label,
    lbl_server4: Label,
    ent_server1: Entry,
    ent_server2: Entry,
    ent_server3: Entry,
    ent_server4: Entry,
    ent_username1: Entry,
    ent_username2: Entry,
    ent_username3: Entry,
    ent_username4: Entry,
    ent_password1: Entry,
    ent_password2: Entry,
    ent_password3: Entry,
    ent_password4: Entry,
    swt_server1: Switch,
    swt_server2: Switch,
    swt_server3: Switch,
    swt_server4: Switch,
}

impl SettingsStunStorage {
    fn new(gtk_builder: &Builder) -> Self {
        SettingsStunStorage {
            lbl_server1: gtk_builder.get_object("lbl_stun1").unwrap(),
            lbl_server2: gtk_builder.get_object("lbl_stun2").unwrap(),
            lbl_server3: gtk_builder.get_object("lbl_stun3").unwrap(),
            lbl_server4: gtk_builder.get_object("lbl_stun4").unwrap(),
            ent_server1: gtk_builder.get_object("ent_stun_server1").unwrap(),
            ent_server2: gtk_builder.get_object("ent_stun_server2").unwrap(),
            ent_server3: gtk_builder.get_object("ent_stun_server3").unwrap(),
            ent_server4: gtk_builder.get_object("ent_stun_server4").unwrap(),
            ent_username1: gtk_builder.get_object("ent_stun_username1").unwrap(),
            ent_username2: gtk_builder.get_object("ent_stun_username2").unwrap(),
            ent_username3: gtk_builder.get_object("ent_stun_username3").unwrap(),
            ent_username4: gtk_builder.get_object("ent_stun_username4").unwrap(),
            ent_password1: gtk_builder.get_object("ent_stun_password1").unwrap(),
            ent_password2: gtk_builder.get_object("ent_stun_password2").unwrap(),
            ent_password3: gtk_builder.get_object("ent_stun_password3").unwrap(),
            ent_password4: gtk_builder.get_object("ent_stun_password4").unwrap(),
            swt_server1: gtk_builder.get_object("swt_stun1").unwrap(),
            swt_server2: gtk_builder.get_object("swt_stun2").unwrap(),
            swt_server3: gtk_builder.get_object("swt_stun3").unwrap(),
            swt_server4: gtk_builder.get_object("swt_stun4").unwrap()
        }
    }
}

#[derive(Clone)]
pub struct SettingsStunWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsStunStorage>
}

impl SettingsStunWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsStunWidget {
            ctx: RefCell::new(SettingsStunStorage::new(gtk_builder))
        };

        result.ctx.borrow().lbl_server1.set_sensitive(false);
        result.ctx.borrow().lbl_server2.set_sensitive(false);
        result.ctx.borrow().lbl_server3.set_sensitive(false);
        result.ctx.borrow().lbl_server4.set_sensitive(false);
        result.ctx.borrow().ent_server1.set_sensitive(false);
        result.ctx.borrow().ent_server2.set_sensitive(false);
        result.ctx.borrow().ent_server3.set_sensitive(false);
        result.ctx.borrow().ent_server4.set_sensitive(false);
        result.ctx.borrow().ent_username1.set_sensitive(false);
        result.ctx.borrow().ent_username2.set_sensitive(false);
        result.ctx.borrow().ent_username3.set_sensitive(false);
        result.ctx.borrow().ent_username4.set_sensitive(false);
        result.ctx.borrow().ent_password1.set_sensitive(false);
        result.ctx.borrow().ent_password2.set_sensitive(false);
        result.ctx.borrow().ent_password3.set_sensitive(false);
        result.ctx.borrow().ent_password4.set_sensitive(false);


        let result_clone = result.clone();
        result.ctx.borrow().swt_server1.connect_property_active_notify(move |swt| {
            result_clone.ctx.borrow().lbl_server1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_server1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_username1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_password1.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_server2.connect_property_active_notify(move |swt| {
            result_clone.ctx.borrow().lbl_server2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_server2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_username2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_password2.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_server3.connect_property_active_notify(move |swt| {
            result_clone.ctx.borrow().lbl_server3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_server3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_username3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_password3.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_server4.connect_property_active_notify(move |swt| {
            result_clone.ctx.borrow().lbl_server4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_server4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_username4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_password4.set_sensitive(swt.get_state());
        });

        result
    }

    pub fn reset(&self) {
        self.ctx.borrow().ent_server1.set_text("");
        self.ctx.borrow().ent_server2.set_text("");
        self.ctx.borrow().ent_server3.set_text("");
        self.ctx.borrow().ent_server4.set_text("");

        self.ctx.borrow().ent_username1.set_text("");
        self.ctx.borrow().ent_username2.set_text("");
        self.ctx.borrow().ent_username3.set_text("");
        self.ctx.borrow().ent_username4.set_text("");

        self.ctx.borrow().ent_password1.set_text("");
        self.ctx.borrow().ent_password2.set_text("");
        self.ctx.borrow().ent_password3.set_text("");
        self.ctx.borrow().ent_password4.set_text("");

        self.ctx.borrow().swt_server1.set_state(false);
        self.ctx.borrow().swt_server2.set_state(false);
        self.ctx.borrow().swt_server3.set_state(false);
        self.ctx.borrow().swt_server4.set_state(false);
    }

    pub fn set_server1(&self, value: String) {
        self.ctx.borrow_mut().ent_server1.set_text(value.as_str());
    }

    pub fn get_server1(&self) -> Option<String> {
        if  self.ctx.borrow().swt_server1.get_state() &
            !self.ctx.borrow().ent_server1.get_text().is_empty() {
            Some(self.ctx.borrow().ent_server1.get_text().to_string().clone())
        } else { None }
    }

    pub fn set_server2(&self, value: String) {
        self.ctx.borrow_mut().ent_server2.set_text(value.as_str());
    }

    pub fn get_server2(&self) -> Option<String> {
        if self.ctx.borrow().swt_server2.get_state() &
           !self.ctx.borrow().ent_server2.get_text().is_empty() {
                Some(self.ctx.borrow().ent_server2.get_text().to_string().clone())
        } else { None }
    }

    pub fn set_server3(&self, value: String) {
        self.ctx.borrow_mut().ent_server3.set_text(value.as_str());
    }

    pub fn get_server3(&self) -> Option<String> {
        if self.ctx.borrow().swt_server3.get_state() &
        !self.ctx.borrow().ent_server3.get_text().is_empty() {
            Some(self.ctx.borrow().ent_server3.get_text().to_string().clone())
        } else { None }
    }

    pub fn set_server4(&self, value: String) {
        self.ctx.borrow_mut().ent_server4.set_text(value.as_str());
    }

    pub fn get_server4(&self) -> Option<String> {
        if self.ctx.borrow().swt_server4.get_state() &
        !self.ctx.borrow().ent_server4.get_text().is_empty() {
            Some(self.ctx.borrow().ent_server4.get_text().to_string().clone())
        } else { None }
    }

    pub fn set_username1(&self, value: String) {
        self.ctx.borrow_mut().ent_username1.set_text(value.as_str());
    }

    pub fn get_username1(&self) -> String {
        self.ctx.borrow().ent_username1.get_text().to_string().clone()
    }

    pub fn set_username2(&self, value: String) {
        self.ctx.borrow_mut().ent_username2.set_text(value.as_str());
    }

    pub fn get_username2(&self) -> String {
        self.ctx.borrow().ent_username2.get_text().to_string().clone()
    }

    pub fn set_username3(&self, value: String) {
        self.ctx.borrow_mut().ent_username3.set_text(value.as_str());
    }

    pub fn get_username3(&self) -> String {
        self.ctx.borrow().ent_username3.get_text().to_string().clone()
    }

    pub fn set_username4(&self, value: String) {
        self.ctx.borrow_mut().ent_username4.set_text(value.as_str());
    }

    pub fn get_username4(&self) -> String {
        self.ctx.borrow().ent_username4.get_text().to_string().clone()
    }

    pub fn set_password1(&self, value: String) {
        self.ctx.borrow_mut().ent_password1.set_text(value.as_str());
    }

    pub fn get_password1(&self) -> String {
        self.ctx.borrow().ent_password1.get_text().to_string().clone()
    }

    pub fn set_password2(&self, value: String) {
        self.ctx.borrow_mut().ent_password2.set_text(value.as_str());
    }

    pub fn get_password2(&self) -> String {
        self.ctx.borrow().ent_password2.get_text().to_string().clone()
    }

    pub fn set_password3(&self, value: String) {
        self.ctx.borrow_mut().ent_password3.set_text(value.as_str());
    }

    pub fn get_password3(&self) -> String {
        self.ctx.borrow().ent_password3.get_text().to_string().clone()
    }

    pub fn set_password4(&self, value: String) {
        self.ctx.borrow_mut().ent_password4.set_text(value.as_str());
    }

    pub fn get_password4(&self) -> String {
        self.ctx.borrow().ent_password4.get_text().to_string().clone()
    }

    pub fn set_state_server1(&self, value: bool) {
        self.ctx.borrow_mut().swt_server1.set_state(value);
    }

    pub fn get_state_server1(&self) -> bool {
        self.ctx.borrow().swt_server1.get_state()
    }

    pub fn set_state_server2(&self, value: bool) {
        self.ctx.borrow_mut().swt_server2.set_state(value);
    }

    pub fn get_state_server2(&self) -> bool {
        self.ctx.borrow().swt_server1.get_state()
    }

    pub fn set_state_server3(&self, value: bool) {
        self.ctx.borrow_mut().swt_server3.set_state(value);
    }

    pub fn get_state_server3(&self) -> bool {
        self.ctx.borrow().swt_server1.get_state()
    }

    pub fn set_state_server4(&self, value: bool) {
        self.ctx.borrow_mut().swt_server4.set_state(value);
    }

    pub fn get_state_server4(&self) -> bool {
        self.ctx.borrow().swt_server1.get_state()
    }
}


impl HelperFileSettings for SettingsStunWidget {

    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        self.set_server1(config.get("stun", "server1").unwrap_or(String::new()));
        self.set_server2(config.get("stun", "server2").unwrap_or(String::new()));
        self.set_server3(config.get("stun", "server3").unwrap_or(String::new()));
        self.set_server4(config.get("stun", "server4").unwrap_or(String::new()));
        self.set_username1(config.get("stun", "username1").unwrap_or(String::new()));
        self.set_username2(config.get("stun", "username2").unwrap_or(String::new()));
        self.set_username3(config.get("stun", "username3").unwrap_or(String::new()));
        self.set_username4(config.get("stun", "username4").unwrap_or(String::new()));
        self.set_password1(config.get("stun", "password1").unwrap_or(String::new()));
        self.set_password2(config.get("stun", "password2").unwrap_or(String::new()));
        self.set_password3(config.get("stun", "password3").unwrap_or(String::new()));
        self.set_password4(config.get("stun", "password4").unwrap_or(String::new()));
        self.set_state_server1(config.get("stun", "state_server1").unwrap_or(String::from("false")).parse().unwrap());
        self.set_state_server2(config.get("stun", "state_server2").unwrap_or(String::from("false")).parse().unwrap());
        self.set_state_server3(config.get("stun", "state_server3").unwrap_or(String::from("false")).parse().unwrap());
        self.set_state_server4(config.get("stun", "state_server4").unwrap_or(String::from("false")).parse().unwrap());
    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        config.set("stun", "server1", Some(self.ctx.borrow().ent_server1.to_string()));
        config.set("stun", "server2", Some(self.ctx.borrow().ent_server2.to_string()));
        config.set("stun", "server3", Some(self.ctx.borrow().ent_server3.to_string()));
        config.set("stun", "server4", Some(self.ctx.borrow().ent_server4.to_string()));
        config.set("stun", "username1", Some(self.get_username1()));
        config.set("stun", "username2", Some(self.get_username2()));
        config.set("stun", "username3", Some(self.get_username3()));
        config.set("stun", "username4", Some(self.get_username4()));
        config.set("stun", "password1", Some(self.get_password1()));
        config.set("stun", "password2", Some(self.get_password2()));
        config.set("stun", "password3", Some(self.get_password3()));
        config.set("stun", "password4", Some(self.get_password4()));
        config.set("stun", "state_server1", Some(self.get_state_server1().to_string()));
        config.set("stun", "state_server2", Some(self.get_state_server2().to_string()));
        config.set("stun", "state_server3", Some(self.get_state_server3().to_string()));
        config.set("stun", "state_server4", Some(self.get_state_server4().to_string()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}

