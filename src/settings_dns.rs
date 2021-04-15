

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
}


impl HelperFileSettings for SettingsDnsWidget {
    fn load(&self, path: PathBuf) {
        todo!()
    }

    fn save(&self, path: PathBuf) {
        todo!()
    }
}


