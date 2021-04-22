

use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsDnsStorage {
    lbl_nameserver1: Label,
    lbl_nameserver2: Label,
    lbl_nameserver3: Label,
    lbl_nameserver4: Label,
    ent_nameserver1: Entry,
    ent_nameserver2: Entry,
    ent_nameserver3: Entry,
    ent_nameserver4: Entry,
    swt_nameserver1: Switch,
    swt_nameserver2: Switch,
    swt_nameserver3: Switch,
    swt_nameserver4: Switch
}


impl SettingsDnsStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsDnsStorage {
            lbl_nameserver1: gtk_builder.get_object("lbl_nameserver1").unwrap(),
            lbl_nameserver2: gtk_builder.get_object("lbl_nameserver2").unwrap(),
            lbl_nameserver3: gtk_builder.get_object("lbl_nameserver3").unwrap(),
            lbl_nameserver4: gtk_builder.get_object("lbl_nameserver4").unwrap(),
            ent_nameserver1: gtk_builder.get_object("ent_nameserver1").unwrap(),
            ent_nameserver2: gtk_builder.get_object("ent_nameserver2").unwrap(),
            ent_nameserver3: gtk_builder.get_object("ent_nameserver3").unwrap(),
            ent_nameserver4: gtk_builder.get_object("ent_nameserver4").unwrap(),
            swt_nameserver1: gtk_builder.get_object("swt_nameserver1").unwrap(),
            swt_nameserver2: gtk_builder.get_object("swt_nameserver2").unwrap(),
            swt_nameserver3: gtk_builder.get_object("swt_nameserver3").unwrap(),
            swt_nameserver4: gtk_builder.get_object("swt_nameserver4").unwrap()
        }
    }
}


#[derive(Clone)]
pub struct SettingsDnsWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsDnsStorage>
}

impl SettingsDnsWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsDnsWidget {
            ctx: RefCell::new(SettingsDnsStorage::new(gtk_builder))
        };

        result.ctx.borrow().lbl_nameserver1.set_sensitive(false);
        result.ctx.borrow().lbl_nameserver2.set_sensitive(false);
        result.ctx.borrow().lbl_nameserver3.set_sensitive(false);
        result.ctx.borrow().lbl_nameserver4.set_sensitive(false);
        result.ctx.borrow().ent_nameserver1.set_sensitive(false);
        result.ctx.borrow().ent_nameserver2.set_sensitive(false);
        result.ctx.borrow().ent_nameserver3.set_sensitive(false);
        result.ctx.borrow().ent_nameserver4.set_sensitive(false);

        let result_clone = result.clone();
        result.ctx.borrow().swt_nameserver1.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_nameserver1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_nameserver1.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_nameserver2.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_nameserver2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_nameserver2.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_nameserver3.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_nameserver3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_nameserver3.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_nameserver4.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_nameserver4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_nameserver4.set_sensitive(swt.get_state());
        });

        result
    }

    pub fn reset(&self) {
        self.ctx.borrow().ent_nameserver1.set_text("");
        self.ctx.borrow().ent_nameserver2.set_text("");
        self.ctx.borrow().ent_nameserver3.set_text("");
        self.ctx.borrow().ent_nameserver4.set_text("");

        self.ctx.borrow().swt_nameserver1.set_state(false);
        self.ctx.borrow().swt_nameserver2.set_state(false);
        self.ctx.borrow().swt_nameserver3.set_state(false);
        self.ctx.borrow().swt_nameserver4.set_state(false);
    }


    pub fn set_nameserver1(&self, value: String) {
        self.ctx.borrow_mut().ent_nameserver1.set_text(value.as_str());
    }

    pub fn get_nameserver1(&self) -> String {
        self.ctx.borrow().ent_nameserver1.get_text().to_string().clone()
    }

    pub fn set_nameserver2(&self, value: String) {
        self.ctx.borrow_mut().ent_nameserver2.set_text(value.as_str());
    }

    pub fn get_nameserver2(&self) -> String {
        self.ctx.borrow().ent_nameserver2.get_text().to_string().clone()
    }

    pub fn set_nameserver3(&self, value: String) {
        self.ctx.borrow_mut().ent_nameserver3.set_text(value.as_str());
    }

    pub fn get_nameserver3(&self) -> String {
        self.ctx.borrow().ent_nameserver3.get_text().to_string().clone()
    }

    pub fn set_nameserver4(&self, value: String) {
        self.ctx.borrow_mut().ent_nameserver4.set_text(value.as_str());
    }

    pub fn get_nameserver4(&self) -> String {
        self.ctx.borrow().ent_nameserver4.get_text().to_string().clone()
    }

    pub fn set_state_nameserver1(&self, value: bool) {
        self.ctx.borrow_mut().swt_nameserver1.set_state(value);
    }

    pub fn get_state_nameserver1(&self) -> bool {
        self.ctx.borrow().swt_nameserver1.get_state()
    }

    pub fn set_state_nameserver2(&self, value: bool) {
        self.ctx.borrow_mut().swt_nameserver2.set_state(value);
    }

    pub fn get_state_nameserver2(&self) -> bool {
        self.ctx.borrow().swt_nameserver2.get_state()
    }

    pub fn set_state_nameserver3(&self, value: bool) {
        self.ctx.borrow_mut().swt_nameserver3.set_state(value);
    }

    pub fn get_state_nameserver3(&self) -> bool {
        self.ctx.borrow().swt_nameserver3.get_state()
    }

    pub fn set_state_nameserver4(&self, value: bool) {
        self.ctx.borrow_mut().swt_nameserver4.set_state(value);
    }

    pub fn get_state_nameserver4(&self) -> bool {
        self.ctx.borrow().swt_nameserver4.get_state()
    }

}


impl HelperFileSettings for SettingsDnsWidget {
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        self.set_nameserver1(config.get("dns", "nameserver1").unwrap());
        self.set_nameserver2(config.get("dns", "nameserver2").unwrap());
        self.set_nameserver3(config.get("dns", "nameserver3").unwrap());
        self.set_nameserver4(config.get("dns", "nameserver4").unwrap());
        self.set_state_nameserver1(config.get("dns", "state_nameserver1").unwrap().parse().unwrap());
        self.set_state_nameserver2(config.get("dns", "state_nameserver2").unwrap().parse().unwrap());
        self.set_state_nameserver3(config.get("dns", "state_nameserver3").unwrap().parse().unwrap());
        self.set_state_nameserver4(config.get("dns", "state_nameserver4").unwrap().parse().unwrap());
    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        config.set("dns", "nameserver1", Some(self.get_nameserver1()));
        config.set("dns", "nameserver2", Some(self.get_nameserver2()));
        config.set("dns", "nameserver3", Some(self.get_nameserver3()));
        config.set("dns", "nameserver4", Some(self.get_nameserver4()));
        config.set("dns", "state_nameserver1", Some(self.get_state_nameserver1().to_string()));
        config.set("dns", "state_nameserver2", Some(self.get_state_nameserver2().to_string()));
        config.set("dns", "state_nameserver3", Some(self.get_state_nameserver3().to_string()));
        config.set("dns", "state_nameserver4", Some(self.get_state_nameserver4().to_string()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}


