

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

}