

use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;

#[derive(Clone)]
pub struct SettingsStunStorage {
    lbl_stun1: Label,
    lbl_stun2: Label,
    lbl_stun3: Label,
    lbl_stun4: Label,
    ent_stun_server1: Entry,
    ent_stun_server2: Entry,
    ent_stun_server3: Entry,
    ent_stun_server4: Entry,
    ent_stun_username1: Entry,
    ent_stun_username2: Entry,
    ent_stun_username3: Entry,
    ent_stun_username4: Entry,
    ent_stun_password1: Entry,
    ent_stun_password2: Entry,
    ent_stun_password3: Entry,
    ent_stun_password4: Entry,
    swt_stun1: Switch,
    swt_stun2: Switch,
    swt_stun3: Switch,
    swt_stun4: Switch,
}

impl SettingsStunStorage {
    fn new(gtk_builder: &Builder) -> Self {
        SettingsStunStorage {
            lbl_stun1: gtk_builder.get_object("lbl_stun1").unwrap(),
            lbl_stun2: gtk_builder.get_object("lbl_stun2").unwrap(),
            lbl_stun3: gtk_builder.get_object("lbl_stun3").unwrap(),
            lbl_stun4: gtk_builder.get_object("lbl_stun4").unwrap(),
            ent_stun_server1: gtk_builder.get_object("ent_stun_server1").unwrap(),
            ent_stun_server2: gtk_builder.get_object("ent_stun_server2").unwrap(),
            ent_stun_server3: gtk_builder.get_object("ent_stun_server3").unwrap(),
            ent_stun_server4: gtk_builder.get_object("ent_stun_server4").unwrap(),
            ent_stun_username1: gtk_builder.get_object("ent_stun_username1").unwrap(),
            ent_stun_username2: gtk_builder.get_object("ent_stun_username2").unwrap(),
            ent_stun_username3: gtk_builder.get_object("ent_stun_username3").unwrap(),
            ent_stun_username4: gtk_builder.get_object("ent_stun_username4").unwrap(),
            ent_stun_password1: gtk_builder.get_object("ent_stun_password1").unwrap(),
            ent_stun_password2: gtk_builder.get_object("ent_stun_password2").unwrap(),
            ent_stun_password3: gtk_builder.get_object("ent_stun_password3").unwrap(),
            ent_stun_password4: gtk_builder.get_object("ent_stun_password4").unwrap(),
            swt_stun1: gtk_builder.get_object("swt_stun1").unwrap(),
            swt_stun2: gtk_builder.get_object("swt_stun2").unwrap(),
            swt_stun3: gtk_builder.get_object("swt_stun3").unwrap(),
            swt_stun4: gtk_builder.get_object("swt_stun4").unwrap()
        }
    }
}

#[derive(Clone)]
pub struct SettingsStunWidget {
    ctx: RefCell<SettingsStunStorage>
}

impl SettingsStunWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsStunWidget {
            ctx: RefCell::new(SettingsStunStorage::new(gtk_builder))
        };

        result.ctx.borrow().lbl_stun1.set_sensitive(false);
        result.ctx.borrow().lbl_stun2.set_sensitive(false);
        result.ctx.borrow().lbl_stun3.set_sensitive(false);
        result.ctx.borrow().lbl_stun4.set_sensitive(false);
        result.ctx.borrow().ent_stun_server1.set_sensitive(false);
        result.ctx.borrow().ent_stun_server2.set_sensitive(false);
        result.ctx.borrow().ent_stun_server3.set_sensitive(false);
        result.ctx.borrow().ent_stun_server4.set_sensitive(false);
        result.ctx.borrow().ent_stun_username1.set_sensitive(false);
        result.ctx.borrow().ent_stun_username2.set_sensitive(false);
        result.ctx.borrow().ent_stun_username3.set_sensitive(false);
        result.ctx.borrow().ent_stun_username4.set_sensitive(false);
        result.ctx.borrow().ent_stun_password1.set_sensitive(false);
        result.ctx.borrow().ent_stun_password2.set_sensitive(false);
        result.ctx.borrow().ent_stun_password3.set_sensitive(false);
        result.ctx.borrow().ent_stun_password4.set_sensitive(false);

        let result_clone = result.clone();
        result.ctx.borrow().swt_stun1.connect_property_active_notify(move |swt| {
            result_clone.ctx.borrow().lbl_stun1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_server1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_username1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_password1.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_stun2.connect_property_active_notify(move |swt| {
            result_clone.ctx.borrow().lbl_stun2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_server2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_username2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_password2.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_stun3.connect_property_active_notify(move |swt| {
            result_clone.ctx.borrow().lbl_stun3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_server3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_username3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_password3.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_stun4.connect_property_active_notify(move |swt| {
            result_clone.ctx.borrow().lbl_stun4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_server4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_username4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_stun_password4.set_sensitive(swt.get_state());
        });

        result
    }
}


impl HelperFileSettings for SettingsStunStorage {
    fn load(&self, path: PathBuf) {
        todo!()
    }

    fn save(&self, path: PathBuf) {
        todo!()
    }
}

