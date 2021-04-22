

use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;



#[derive(Clone)]
pub struct SettingsProxyStorage {
    lbl_proxy1: Label,
    lbl_proxy2: Label,
    lbl_proxy3: Label,
    lbl_proxy4: Label,
    ent_proxy1: Entry,
    ent_proxy2: Entry,
    ent_proxy3: Entry,
    ent_proxy4: Entry,
    ent_username1: Entry,
    ent_username2: Entry,
    ent_username3: Entry,
    ent_username4: Entry,
    ent_password1: Entry,
    ent_password2: Entry,
    ent_password3: Entry,
    ent_password4: Entry,
    swt_proxy1: Switch,
    swt_proxy2: Switch,
    swt_proxy3: Switch,
    swt_proxy4: Switch,

}

impl SettingsProxyStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsProxyStorage {
            lbl_proxy1: gtk_builder.get_object("lbl_outbound_proxy1").unwrap(),
            lbl_proxy2: gtk_builder.get_object("lbl_outbound_proxy2").unwrap(),
            lbl_proxy3: gtk_builder.get_object("lbl_outbound_proxy3").unwrap(),
            lbl_proxy4: gtk_builder.get_object("lbl_outbound_proxy4").unwrap(),
            ent_proxy1: gtk_builder.get_object("ent_outbound_proxy1").unwrap(),
            ent_proxy2: gtk_builder.get_object("ent_outbound_proxy2").unwrap(),
            ent_proxy3: gtk_builder.get_object("ent_outbound_proxy3").unwrap(),
            ent_proxy4: gtk_builder.get_object("ent_outbound_proxy4").unwrap(),
            ent_username1: gtk_builder.get_object("ent_outbound_proxy_username1").unwrap(),
            ent_username2: gtk_builder.get_object("ent_outbound_proxy_username2").unwrap(),
            ent_username3: gtk_builder.get_object("ent_outbound_proxy_username3").unwrap(),
            ent_username4: gtk_builder.get_object("ent_outbound_proxy_username4").unwrap(),
            ent_password1: gtk_builder.get_object("ent_outbound_proxy_password1").unwrap(),
            ent_password2: gtk_builder.get_object("ent_outbound_proxy_password2").unwrap(),
            ent_password3: gtk_builder.get_object("ent_outbound_proxy_password3").unwrap(),
            ent_password4: gtk_builder.get_object("ent_outbound_proxy_password4").unwrap(),
            swt_proxy1: gtk_builder.get_object("swt_outbound_proxy1").unwrap(),
            swt_proxy2: gtk_builder.get_object("swt_outbound_proxy2").unwrap(),
            swt_proxy3: gtk_builder.get_object("swt_outbound_proxy3").unwrap(),
            swt_proxy4: gtk_builder.get_object("swt_outbound_proxy4").unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct SettingsProxyWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsProxyStorage>
}

impl SettingsProxyWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsProxyWidget {
            ctx: RefCell::new(SettingsProxyStorage::new(gtk_builder))
        };

        result.ctx.borrow().lbl_proxy1.set_sensitive(false);
        result.ctx.borrow().lbl_proxy2.set_sensitive(false);
        result.ctx.borrow().lbl_proxy3.set_sensitive(false);
        result.ctx.borrow().lbl_proxy4.set_sensitive(false);
        result.ctx.borrow().ent_proxy1.set_sensitive(false);
        result.ctx.borrow().ent_proxy2.set_sensitive(false);
        result.ctx.borrow().ent_proxy3.set_sensitive(false);
        result.ctx.borrow().ent_proxy4.set_sensitive(false);
        result.ctx.borrow().ent_username1.set_sensitive(false);
        result.ctx.borrow().ent_username2.set_sensitive(false);
        result.ctx.borrow().ent_username3.set_sensitive(false);
        result.ctx.borrow().ent_username4.set_sensitive(false);
        result.ctx.borrow().ent_password1.set_sensitive(false);
        result.ctx.borrow().ent_password2.set_sensitive(false);
        result.ctx.borrow().ent_password3.set_sensitive(false);
        result.ctx.borrow().ent_password4.set_sensitive(false);

        let result_clone = result.clone();
        result.ctx.borrow().swt_proxy1.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_proxy1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_proxy1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_username1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_password1.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_proxy2.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_proxy2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_proxy2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_username2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_password2.set_sensitive(swt.get_state());

        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_proxy3.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_proxy3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_proxy3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_username3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_password3.set_sensitive(swt.get_state());

        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_proxy4.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_proxy4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_proxy4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_username4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_password4.set_sensitive(swt.get_state());

        });

        result.reset();

        result
    }

    pub fn reset(&self) {
        self.ctx.borrow().ent_proxy1.set_text("");
        self.ctx.borrow().ent_proxy2.set_text("");
        self.ctx.borrow().ent_proxy3.set_text("");
        self.ctx.borrow().ent_proxy4.set_text("");

        self.ctx.borrow().ent_username1.set_text("");
        self.ctx.borrow().ent_username2.set_text("");
        self.ctx.borrow().ent_username3.set_text("");
        self.ctx.borrow().ent_username4.set_text("");

        self.ctx.borrow().ent_password1.set_text("");
        self.ctx.borrow().ent_password2.set_text("");
        self.ctx.borrow().ent_password3.set_text("");
        self.ctx.borrow().ent_password4.set_text("");

        self.ctx.borrow().swt_proxy1.set_state(false);
        self.ctx.borrow().swt_proxy2.set_state(false);
        self.ctx.borrow().swt_proxy3.set_state(false);
        self.ctx.borrow().swt_proxy4.set_state(false);
    }

    pub fn set_proxy1(&self, value: String) {
        self.ctx.borrow_mut().ent_proxy1.set_text(value.as_str());
    }

    pub fn get_proxy1(&self) -> String {
        self.ctx.borrow().ent_proxy1.get_text().to_string().clone()
    }

    pub fn set_proxy2(&self, value: String) {
        self.ctx.borrow_mut().ent_proxy2.set_text(value.as_str());
    }

    pub fn get_proxy2(&self) -> String {
        self.ctx.borrow().ent_proxy2.get_text().to_string().clone()
    }

    pub fn set_proxy3(&self, value: String) {
        self.ctx.borrow_mut().ent_proxy3.set_text(value.as_str());
    }

    pub fn get_proxy3(&self) -> String {
        self.ctx.borrow().ent_proxy3.get_text().to_string().clone()
    }

    pub fn set_proxy4(&self, value: String) {
        self.ctx.borrow_mut().ent_proxy4.set_text(value.as_str());
    }

    pub fn get_proxy4(&self) -> String {
        self.ctx.borrow().ent_proxy4.get_text().to_string().clone()
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

    pub fn set_state_proxy1(&self, value: bool) {
        self.ctx.borrow_mut().swt_proxy1.set_state(value);
    }

    pub fn get_state_proxy1(&self) -> bool {
        self.ctx.borrow().swt_proxy1.get_state()
    }

    pub fn set_state_proxy2(&self, value: bool) {
        self.ctx.borrow_mut().swt_proxy2.set_state(value);
    }

    pub fn get_state_proxy2(&self) -> bool {
        self.ctx.borrow().swt_proxy2.get_state()
    }

    pub fn set_state_proxy3(&self, value: bool) {
        self.ctx.borrow_mut().swt_proxy3.set_state(value);
    }

    pub fn get_state_proxy3(&self) -> bool {
        self.ctx.borrow().swt_proxy3.get_state()
    }

    pub fn set_state_proxy4(&self, value: bool) {
        self.ctx.borrow_mut().swt_proxy4.set_state(value);
    }

    pub fn get_state_proxy4(&self) -> bool {
        self.ctx.borrow().swt_proxy4.get_state()
    }
}

impl HelperFileSettings for SettingsProxyWidget {
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        self.set_proxy1(config.get("proxy", "proxy1").unwrap());
        self.set_proxy2(config.get("proxy", "proxy2").unwrap());
        self.set_proxy3(config.get("proxy", "proxy3").unwrap());
        self.set_proxy4(config.get("proxy", "proxy4").unwrap());
        self.set_username1(config.get("proxy", "username1").unwrap());
        self.set_username2(config.get("proxy", "username2").unwrap());
        self.set_username3(config.get("proxy", "username3").unwrap());
        self.set_username4(config.get("proxy", "username4").unwrap());
        self.set_password1(config.get("proxy", "password1").unwrap());
        self.set_password2(config.get("proxy", "password2").unwrap());
        self.set_password3(config.get("proxy", "password3").unwrap());
        self.set_password4(config.get("proxy", "password4").unwrap());
        self.set_state_proxy1(config.get("proxy", "state_proxy1").unwrap().parse().unwrap());
        self.set_state_proxy2(config.get("proxy", "state_proxy2").unwrap().parse().unwrap());
        self.set_state_proxy3(config.get("proxy", "state_proxy3").unwrap().parse().unwrap());
        self.set_state_proxy4(config.get("proxy", "state_proxy4").unwrap().parse().unwrap());

    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        config.set("proxy", "proxy1", Some(self.get_proxy1()));
        config.set("proxy", "proxy2", Some(self.get_proxy2()));
        config.set("proxy", "proxy3", Some(self.get_proxy3()));
        config.set("proxy", "proxy4", Some(self.get_proxy4()));
        config.set("proxy", "username1", Some(self.get_username1()));
        config.set("proxy", "username2", Some(self.get_username2()));
        config.set("proxy", "username3", Some(self.get_username3()));
        config.set("proxy", "username4", Some(self.get_username4()));
        config.set("proxy", "password1", Some(self.get_password1()));
        config.set("proxy", "password2", Some(self.get_password2()));
        config.set("proxy", "password3", Some(self.get_password3()));
        config.set("proxy", "password4", Some(self.get_password4()));
        config.set("proxy", "state_proxy1", Some(self.get_state_proxy1().to_string()));
        config.set("proxy", "state_proxy2", Some(self.get_state_proxy2().to_string()));
        config.set("proxy", "state_proxy3", Some(self.get_state_proxy3().to_string()));
        config.set("proxy", "state_proxy4", Some(self.get_state_proxy4().to_string()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}

