

use super::gtk::prelude::*;

use super::gtk::{Switch, Button, Entry, SpinButton, Label, Builder, Notebook};
use super::glib::clone;

use std::cell::RefCell;


#[derive(Clone)]
pub struct SettingsWidgetStorage {
    // gtk notebook
    ntbk_settings: Notebook,
    // call tab section
    // swt_autoanswer: gtk::Switch,
    // lbl_autoanswer: gtk::Label,
    // Stun tab section
    lbl_use_turn: gtk::Label,
    lbl_turn_tcp: gtk::Label,
    lbl_turn_rtcp_multiplexing: gtk::Label,
    lbl_turn_server: gtk::Label,
    lbl_turn_port: gtk::Label,
    lbl_turn_username: gtk::Label,
    lbl_turn_password: gtk::Label,
    lbl_turn_keyring: gtk::Label,
    swt_use_turn: gtk::Switch,
    swt_turn_tcp: gtk::Switch,
    swt_turn_rtcp_multiplexing: gtk::Switch,
    spn_turn_port: gtk::SpinButton,
    ent_turn_server: gtk::Entry,
    ent_turn_username: gtk::Entry,
    ent_turn_password: gtk::Entry,
    cmb_turn_keyring: gtk::ComboBoxText,
    btn_turn_save: gtk::Button,
    btn_turn_reset: gtk::Button,
    // ice section
    lbl_use_ice: gtk::Label,
    lbl_ice_use_rtcp: gtk::Label,
    lbl_ice_reg_nomination: gtk::Label,
    lbl_ice_trickle_method: gtk::Label,
    lbl_ice_max_hosts: gtk::Label,
    swt_use_ice: gtk::Switch,
    swt_ice_rtcp: gtk::Switch,
    cmb_ice_reg_nomination: gtk::ComboBoxText,
    cmb_ice_trickle_method: gtk::ComboBoxText,
    spn_ice_max_hosts: gtk::SpinButton
}

impl SettingsWidgetStorage {
    pub fn new(gtk_builder: &gtk::Builder) -> Self {
        SettingsWidgetStorage {
            // gtk notebook
            ntbk_settings: gtk_builder.get_object("ntbk_settings").unwrap(),
            // // call
            // swt_autoanswer: gtk_builder.get_object("swt_autoanswer").unwrap(),
            // lbl_autoanswer: gtk_builder.get_object("lbl_autoanswer").unwrap(),
            // turn section
            lbl_use_turn: gtk_builder.get_object("lbl_use_turn").unwrap(),
            lbl_turn_tcp: gtk_builder.get_object("lbl_turn_tcp").unwrap(),
            lbl_turn_rtcp_multiplexing: gtk_builder.get_object("lbl_turn_rtcp_multiplexing").unwrap(),
            lbl_turn_server: gtk_builder.get_object("lbl_turn_server").unwrap(),
            lbl_turn_port: gtk_builder.get_object("lbl_turn_port").unwrap(),
            lbl_turn_username: gtk_builder.get_object("lbl_turn_username").unwrap(),
            lbl_turn_password: gtk_builder.get_object("lbl_turn_password").unwrap(),
            lbl_turn_keyring: gtk_builder.get_object("lbl_turn_keyring").unwrap(),
            swt_use_turn: gtk_builder.get_object("swt_use_turn").unwrap(),
            swt_turn_tcp: gtk_builder.get_object("swt_turn_tcp").unwrap(),
            swt_turn_rtcp_multiplexing: gtk_builder.get_object("swt_turn_rtcp_multiplexing").unwrap(),
            spn_turn_port: gtk_builder.get_object("spn_turn_port").unwrap(),
            ent_turn_server: gtk_builder.get_object("ent_turn_server").unwrap(),
            ent_turn_username: gtk_builder.get_object("ent_turn_username").unwrap(),
            ent_turn_password: gtk_builder.get_object("ent_turn_password").unwrap(),
            cmb_turn_keyring: gtk_builder.get_object("cmb_turn_keyring").unwrap(),
            btn_turn_save: gtk_builder.get_object("btn_turn_save").unwrap(),
            btn_turn_reset: gtk_builder.get_object("btn_turn_reset").unwrap(),
            // ice section
            lbl_use_ice: gtk_builder.get_object("lbl_use_ice").unwrap(),
            lbl_ice_use_rtcp: gtk_builder.get_object("lbl_ice_rtcp").unwrap(),
            lbl_ice_reg_nomination: gtk_builder.get_object("lbl_ice_reg_nomination").unwrap(),
            lbl_ice_trickle_method: gtk_builder.get_object("lbl_ice_trickle_method").unwrap(),
            lbl_ice_max_hosts: gtk_builder.get_object("lbl_ice_max_hosts").unwrap(),
            swt_use_ice: gtk_builder.get_object("swt_use_ice").unwrap(),
            swt_ice_rtcp: gtk_builder.get_object("swt_ice_rtcp").unwrap(),
            cmb_ice_reg_nomination: gtk_builder.get_object("cmb_ice_reg_nomination").unwrap(),
            cmb_ice_trickle_method: gtk_builder.get_object("cmb_ice_trickle_method").unwrap(),
            spn_ice_max_hosts: gtk_builder.get_object("spn_ice_max_hosts").unwrap()
        }
    }
}

#[derive(Clone)]
pub struct SettingsWidget {
    ctx: RefCell<SettingsWidgetStorage>
}


impl SettingsWidget {

    pub fn new(gtk_builder: &gtk::Builder) -> Self {
        // inner data just borrow not mutate
        SettingsWidget {
            ctx: RefCell::new(SettingsWidgetStorage::new(gtk_builder)),
        }
    }
}