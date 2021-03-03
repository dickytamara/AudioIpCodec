

use super::gtk::prelude::*;

use super::gtk::{Switch, Button, Entry, SpinButton, Label, Builder, Notebook};
use super::glib::clone;


#[derive(Clone)]
pub struct SettingsWidget {
    // gtk notebook
    ntbk_settings: Notebook,
    // call tab section
    swt_autoanswer: gtk::Switch,
    lbl_autoanswer: gtk::Label,
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

impl SettingsWidget {
    pub fn new(gtk_builder: &gtk::Builder) -> SettingsWidget {
        SettingsWidget {
            // gtk notebook
            ntbk_settings: gtk_builder.get_object("ntbk_settings").unwrap(),
            // call
            swt_autoanswer: gtk_builder.get_object("swt_autoanswer").unwrap(),
            lbl_autoanswer: gtk_builder.get_object("lbl_autoanswer").unwrap(),
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

    pub fn init(&self) {

        // set spin button turn port server
        self.spn_turn_port.set_digits(0);
        self.spn_turn_port.set_range(3478_f64, 65535_f64);
        self.spn_turn_port.set_increments(1_f64, 5_f64);


        // set spin button ice max hosts sever
        self.spn_ice_max_hosts.set_digits(0);
        self.spn_ice_max_hosts.set_range(-1_f64, 256_f64);
        self.spn_ice_max_hosts.set_increments(1_f64, 5_f64);

        // reset widget
        self.sub_widget_turn_reset();
        self.sub_widget_ice_reset();

        // toggle use turn
        let wid = self.clone();
        self.swt_use_turn.connect_property_active_notify( move | s | {
            wid.widget_set_use_turn(s.get_state());
        });

        // toggle use ice
        let wid = self.clone();
        self.swt_use_ice.connect_property_active_notify(move | s | {
            wid.widget_set_use_ice(s.get_state());
        });

        let wid = self.clone();
        let notebook = self.ntbk_settings.clone();
        self.btn_turn_reset.connect_clicked(move |_| {
            match notebook.get_current_page() {
                Some(x) => {
                    match x {
                        0 => wid.sub_widget_call_reset(),
                        1 => wid.sub_widget_turn_reset(),
                        2 => wid.sub_widget_ice_reset(),
                        _ => panic!("")
                    }
                }
                _ => panic!("Setting page not have sub")
            }
            // wid.sub_widget_turn_reset();
        });

    }

    pub fn sub_widget_turn_reset(&self) {
        // set default value for turn properties
        self.swt_use_turn.set_state(false);
        self.swt_turn_tcp.set_state(false);
        self.swt_turn_rtcp_multiplexing.set_state(false);

        self.spn_turn_port.set_value(3478_f64);
        self.ent_turn_server.set_text("");
        self.ent_turn_username.set_text("");
        self.ent_turn_password.set_text("");
        self.cmb_turn_keyring.set_active_id(Some("SDES"));
        self.widget_set_use_turn(false);
    }

    pub fn sub_widget_ice_reset(&self) {
        // set default value for ice properties
        self.swt_use_ice.set_state(false);
        self.swt_ice_rtcp.set_state(false);
        self.cmb_ice_reg_nomination.set_active_id(Some("Agresive"));
        self.cmb_ice_trickle_method.set_active_id(Some("Disabled"));
        self.spn_ice_max_hosts.set_value(-1_f64);
        self.widget_set_use_ice(false);
    }

    pub fn sub_widget_call_reset(&self) {
        self.swt_autoanswer.set_state(false);
    }

    pub fn widget_set_use_turn(&self, value: bool) {
        self.swt_turn_tcp.set_sensitive(value);
        self.swt_turn_rtcp_multiplexing.set_sensitive(value);
        self.ent_turn_server.set_sensitive(value);
        self.spn_turn_port.set_sensitive(value);
        self.ent_turn_username.set_sensitive(value);
        self.ent_turn_password.set_sensitive(value);
        self.cmb_turn_keyring.set_sensitive(value);
    }

    pub fn widget_set_use_ice(&self, value: bool) {
        self.swt_ice_rtcp.set_sensitive(value);
        self.cmb_ice_reg_nomination.set_sensitive(value);
        self.cmb_ice_trickle_method.set_sensitive(value);
        self.spn_ice_max_hosts.set_sensitive(value);
    }

    // auto answer toggle button
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
        self.swt_turn_tcp.set_state(value);
    }

    pub fn get_turn_use_tcp(&self) -> bool {
        self.swt_turn_tcp.get_state()
    }

    pub fn set_turn_use_rtcp_multiplexing(&self, value: bool) {
        self.swt_turn_rtcp_multiplexing.set_state(value);
    }

    pub fn get_turn_use_rtcp_multiplexing(&self) -> bool {
        self.swt_turn_rtcp_multiplexing.get_state()
    }

    pub fn get_stun_server(&self) -> String {
        self.ent_turn_server.get_text().to_string().clone()
    }

    pub fn set_stun_server(&self, value: String) {
        self.ent_turn_server.set_text(value.as_str());
    }

    pub fn get_stun_port(&self) -> f64 {
        self.spn_turn_port.get_value()
    }

    pub fn set_stun_port(&self, value: f64) {
        self.spn_turn_port.set_value(value);
    }

    pub fn set_stun_username(&self, value: String) {
        self.ent_turn_username.set_text(value.as_str());
    }

    pub fn get_stun_username(&self) -> String {
        self.ent_turn_username.get_text().to_string().clone()
    }

    pub fn set_stun_password(&self, value: String) {
        self.ent_turn_password.set_text(value.as_str());
    }

    pub fn get_stun_password(&self) -> String {
        self.ent_turn_password.get_text().to_string().clone()
    }

    pub fn set_use_ice(&self, value: bool) {
        self.swt_use_ice.set_state(value);
    }

    pub fn get_use_ice(&self) -> bool {
        self.swt_use_ice.get_state()
    }

    pub fn set_ice_use_rtcp(&self, value: bool) {
        self.swt_ice_rtcp.set_state(value);
    }

    pub fn get_ice_use_rtcp(&self) -> bool {
        self.swt_ice_rtcp.get_state()
    }



}