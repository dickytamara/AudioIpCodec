

use super::gtk::prelude::*;
use super::gio::prelude::*;


use super::gtk::{Label, Entry, Button, Builder};
use super::glib::clone;


pub struct AccountWidget {
    lbl_sip_url: gtk::Label,
    lbl_registrar_url: gtk::Label,
    lbl_realm: gtk::Label,
    lbl_username: gtk::Label,
    lbl_password: gtk::Label,
    ent_sip_url: gtk::Entry,
    ent_registrar_url: gtk::Entry,
    ent_realm: gtk::Entry,
    ent_username: gtk::Entry,
    ent_password: gtk::Entry,
    btn_connect: gtk::Button,
    btn_save: gtk::Button
}


impl AccountWidget {

    pub fn new (gtk_builder: &gtk::Builder) -> AccountWidget {
        AccountWidget {
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
            btn_save: gtk_builder.get_object("btn_account_save").unwrap()
        }
    }


    pub fn init(&self) {
        // initialize default event

    }

    pub fn get_sip_url (&self) -> String {
        self.ent_sip_url.get_text().to_string().clone()
    }

    pub fn get_registrar_url (&self) -> String {
        self.ent_registrar_url.get_text().to_string().clone()
    }

    pub fn get_realm (&self) -> String {
        self.ent_realm.get_text().to_string().clone()
    }

    pub fn get_username (&self) -> String {
        self.ent_username.get_text().to_string().clone()
    }

    pub fn get_password (&self) -> String {
        self.ent_username.get_text().to_string().clone()
    }

}

