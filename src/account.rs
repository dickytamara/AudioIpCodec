

use super::gtk::prelude::*;
use super::gtk::{Label, Entry, Button, Builder};
use std::cell::RefCell;


#[derive(Clone)]
pub struct AccountStorage {
    lbl_sip_url: Label,
    lbl_registrar_url: Label,
    lbl_realm: Label,
    lbl_username: Label,
    lbl_password: Label,
    ent_sip_url: Entry,
    ent_registrar_url: Entry,
    ent_realm: Entry,
    ent_username: Entry,
    ent_password: Entry,
    btn_connect: Button,
    btn_save: Button,
    btn_reset: Button
}

impl AccountStorage {
    pub fn new (gtk_builder: &Builder) -> Self {
        AccountStorage {
            lbl_sip_url: gtk_builder.get_object("lbl_sip_url").unwrap(),
            lbl_registrar_url: gtk_builder.get_object("lbl_registrar_url").unwrap(),
            lbl_realm: gtk_builder.get_object("lbl_realm").unwrap(),
            lbl_username: gtk_builder.get_object("lbl_username").unwrap(),
            lbl_password: gtk_builder.get_object("lbl_password").unwrap(),
            ent_sip_url: gtk_builder.get_object("ent_sip_url").unwrap(),
            ent_registrar_url: gtk_builder.get_object("ent_registrar_url").unwrap(),
            ent_realm: gtk_builder.get_object("ent_realm").unwrap(),
            ent_username: gtk_builder.get_object("ent_username").unwrap(),
            ent_password: gtk_builder.get_object("ent_password").unwrap(),
            btn_connect:  gtk_builder.get_object("btn_account_connect").unwrap(),
            btn_save: gtk_builder.get_object("btn_account_save").unwrap(),
            btn_reset: gtk_builder.get_object("btn_account_reset").unwrap()
        }
    }
}

#[derive(Clone)]
pub struct AccountWidget {
    ctx: RefCell<AccountStorage>
}

impl AccountWidget {

    pub fn new (gtk_builder: &gtk::Builder) -> Self {
        let result = AccountWidget {
            ctx: RefCell::new(AccountStorage::new(gtk_builder))
        };

        let widget = result.clone();
        result.ctx.borrow().btn_reset.connect_clicked( move |_| {
            widget.reset();
        });

        result
    }

    pub fn reset(&self) {
        // reset value
        self.ctx.borrow().ent_sip_url.set_text("");
        self.ctx.borrow().ent_registrar_url.set_text("");
        self.ctx.borrow().ent_realm.set_text("");
        self.ctx.borrow().ent_username.set_text("");
        self.ctx.borrow().ent_password.set_text("");
    }

    pub fn get_sip_url (&self) -> String {
        self.ctx.borrow().ent_sip_url.get_text().to_string().clone()
    }

    pub fn set_sip_url (&self, value: &str) {
        self.ctx.borrow().ent_sip_url.set_text(value)
    }

    pub fn get_registrar_url (&self) -> String {
        self.ctx.borrow().ent_registrar_url.get_text().to_string().clone()
    }

    pub fn set_registrar_url (&self, value: &str) {
        self.ctx.borrow().ent_registrar_url.set_text(value);
    }

    pub fn get_realm (&self) -> String {
        self.ctx.borrow().ent_realm.get_text().to_string().clone()
    }

    pub fn set_realm (&self, value: &str) {
        self.ctx.borrow().ent_realm.set_text(value);
    }

    pub fn get_username (&self) -> String {
        self.ctx.borrow().ent_username.get_text().to_string().clone()
    }

    pub fn set_username (&self, value: &str) {
        self.ctx.borrow().ent_username.set_text(value);
    }

    pub fn get_password (&self) -> String {
        self.ctx.borrow().ent_password.get_text().to_string().clone()
    }

    pub fn set_password(&self, value: &str) {
        self.ctx.borrow().ent_password.set_text(value);
    }

    // event on btn save clicked pass closure
    // at outer level
    pub fn on_btn_save_clicked<F: Fn() +'static> (&self, callback: F) {
        self.ctx.borrow().btn_save.connect_clicked( move |_| {
            callback();
        });
    }

    // event on btn connect clicked to pass closure at outer level
    pub fn on_btn_connect_clicked<F: Fn() +'static> (&self, callback: F) {
        self.ctx.borrow().btn_connect.connect_clicked( move |_| {
            callback();
        });
    }

}


