

use super::gtk::prelude::*;

use super::gtk::{Switch, Button, Entry, SpinButton, Label, Builder};


pub struct SettingsWidget {
    // call tab section
    swt_autoanswer: gtk::Switch,
    lbl_autoanswer: gtk::Label,
    // Stun tab section
    lbl_use_turn: gtk::Label,
    lbl_use_tcp: gtk::Label,
    lbl_rtcp_multiplexing: gtk::Label,
    lbl_stun_server: gtk::Label,
    lbl_stun_port: gtk::Label,
    lbl_stun_username: gtk::Label,
    lbl_stun_password: gtk::Label,
    swt_use_turn: gtk::Switch,
    swt_use_tcp: gtk::Switch,
    swt_rtcp_multiplexing: gtk::Switch,
    spn_stun_port: gtk::SpinButton,
    ent_stun_server: gtk::Entry,
    ent_stun_username: gtk::Entry,
    ent_stun_password: gtk::Entry,
    btn_stun_save: gtk::Button,
    btn_stun_reset: gtk::Button
    // ice section
}

impl SettingsWidget {
    pub fn new(gtk_builder: &gtk::Builder) -> SettingsWidget {
        SettingsWidget {
            swt_autoanswer:  gtk_builder.get_object("swt_autoanswer").unwrap(),
            lbl_autoanswer: gtk_builder.get_object("lbl_autoanswer").unwrap(),
            lbl_use_turn: gtk_builder.get_object("lbl_use_turn").unwrap(),
            lbl_use_tcp: gtk_builder.get_object("lbl_use_tcp").unwrap(),
            lbl_rtcp_multiplexing: gtk_builder.get_object("lbl_rtcp_multiplexing").unwrap(),
            lbl_stun_server: gtk_builder.get_object("lbl_stun_server").unwrap(),
            lbl_stun_port: gtk_builder.get_object("lbl_stun_port").unwrap(),
            lbl_stun_username: gtk_builder.get_object("lbl_username").unwrap(),
            lbl_stun_password: gtk_builder.get_object("lbl_stun_password").unwrap(),
            swt_use_turn:  gtk_builder.get_object("swt_use_turn").unwrap(),
            swt_use_tcp:  gtk_builder.get_object("swt_use_tcp").unwrap(),
            swt_rtcp_multiplexing:  gtk_builder.get_object("swt_rtcp_multiplexing").unwrap(),
            spn_stun_port: gtk_builder.get_object("spn_stun_port").unwrap(),
            ent_stun_server: gtk_builder.get_object("ent_stun_server").unwrap(),
            ent_stun_username: gtk_builder.get_object("ent_stun_username").unwrap(),
            ent_stun_password: gtk_builder.get_object("ent_stun_password").unwrap(),
            btn_stun_save:  gtk_builder.get_object("btn_stun_save").unwrap(),
            btn_stun_reset: gtk_builder.get_object("btn_stun_reset").unwrap(),
        }
    }

    pub fn init(&self) {


    }

    pub fn set_account_autoanswer(&self, value: bool) {
        self.swt_autoanswer.set_state(value);
    }

    pub fn get_account_autoanswer(&self) -> bool {
        self.swt_autoanswer.get_state()
    }

    pub fn set_use_turn(&self, value: bool) {
        self.swt_use_turn.set_state(value);
    }

    pub fn get_use_turn(&self) -> bool {
        self.swt_use_turn.get_state()
    }

    pub fn set_turn_use_tcp(&self, value: bool) {
        self.swt_use_tcp.set_state(value);
    }

    pub fn get_turn_use_tcp(&self) -> bool {
        self.swt_use_tcp.get_state()
    }

    pub fn set_turn_use_rtcp_multiplexing(&self, value: bool) {
        self.swt_rtcp_multiplexing.set_state(value);
    }

    pub fn get_turn_use_rtcp_multiplexing(&self) -> bool {
        self.swt_rtcp_multiplexing.get_state()
    }

    pub fn get_stun_server(&self) -> String {
        self.ent_stun_server.get_text().to_string().clone()
    }

    pub fn set_stun_server(&self, value: String) {
        self.ent_stun_server.set_text(value.as_str());
    }

    pub fn get_stun_port(&self) -> f64 {
        self.spn_stun_port.get_value()
    }

    pub fn set_stun_port(&self, value: f64) {
        self.spn_stun_port.set_value(value);
    }

    pub fn set_stun_username(&self, value: String) {
        self.ent_stun_username.set_text(value.as_str());
    }

    pub fn get_stun_username(&self) -> String {
        self.ent_stun_username.get_text().to_string().clone()
    }

    pub fn set_stun_password(&self, value: String) {
        self.ent_stun_password.set_text(value.as_str());
    }

    pub fn get_stun_password(&self) -> String {
        self.ent_stun_password.get_text().to_string().clone()
    }

}