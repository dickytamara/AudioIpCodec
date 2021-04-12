
use gtk::prelude::*;
use gtk::{Label, SpinButton, ComboBoxText, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsTurnWidgetStorage {
    lbl_use_turn: Label,
    lbl_turn_tcp: Label,
    lbl_turn_rtcp_multiplexing: Label,
    lbl_turn_server: Label,
    lbl_turn_port: Label,
    lbl_turn_username: Label,
    lbl_turn_password: Label,
    lbl_turn_keyring: Label,
    swt_use_turn: Switch,
    swt_turn_tcp: Switch,
    swt_turn_rtcp_multiplexing: Switch,
    spn_turn_port: SpinButton,
    ent_turn_server: Entry,
    ent_turn_username: Entry,
    ent_turn_password: Entry,
    cmb_turn_keyring: ComboBoxText,
}

impl SettingsTurnWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsTurnWidgetStorage {
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
        }
    }
}


#[derive(Clone)]
pub struct SettingsTurnWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsTurnWidgetStorage>
}

impl SettingsTurnWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsTurnWidget {
            ctx: RefCell::new(SettingsTurnWidgetStorage::new(gtk_builder)),
        };

        // set spin button turn port server
        result.ctx.borrow().spn_turn_port.set_digits(0);
        result.ctx.borrow().spn_turn_port.set_range(3478_f64, 65535_f64);
        result.ctx.borrow().spn_turn_port.set_increments(1_f64, 5_f64);

        let this = result.clone();
        result.ctx.borrow().swt_use_turn.connect_property_active_notify(move |s| {
            this.set_use_turn(s.get_state())
        });

        result.set_use_turn(false);
        result
    }

    pub fn reset(&self) {
        // set default value for turn properties
        let context = self.ctx.borrow();
        context.swt_use_turn.set_state(false);
        context.swt_turn_tcp.set_state(false);
        context.swt_turn_rtcp_multiplexing.set_state(false);

        context.spn_turn_port.set_value(3478_f64);
        context.ent_turn_server.set_text("");
        context.ent_turn_username.set_text("");
        context.ent_turn_password.set_text("");
        context.cmb_turn_keyring.set_active_id(Some("SDES"));
        self.set_use_turn(false);
    }

    pub fn set_use_turn(&self, value: bool) {
        let context = self.ctx.borrow();
        context.swt_turn_tcp.set_sensitive(value);
        context.swt_turn_rtcp_multiplexing.set_sensitive(value);
        context.ent_turn_server.set_sensitive(value);
        context.spn_turn_port.set_sensitive(value);
        context.ent_turn_username.set_sensitive(value);
        context.ent_turn_password.set_sensitive(value);
        context.cmb_turn_keyring.set_sensitive(value);
    }

    pub fn get_use_turn(&self) -> bool {
        self.ctx.borrow().swt_use_turn.get_state()
    }

    pub fn set_use_tcp(&self, value: bool) {
        self.ctx.borrow().swt_turn_tcp.set_state(value);
    }

    pub fn get_use_tcp(&self) -> bool {
        self.ctx.borrow().swt_turn_tcp.get_state()
    }

    pub fn set_use_rtcp_multiplexing(&self, value: bool) {
        self.ctx.borrow().swt_turn_rtcp_multiplexing.set_state(value);
    }

    pub fn get_use_rtcp_multiplexing(&self) -> bool {
        self.ctx.borrow().swt_turn_rtcp_multiplexing.get_state()
    }

    pub fn get_server(&self) -> String {
        self.ctx.borrow().ent_turn_server.get_text().to_string().clone()
    }

    pub fn set_server(&self, value: String) {
        self.ctx.borrow().ent_turn_server.set_text(value.as_str());
    }

    pub fn get_port(&self) -> f64 {
        self.ctx.borrow().spn_turn_port.get_value()
    }

    pub fn set_port(&self, value: f64) {
        self.ctx.borrow().spn_turn_port.set_value(value);
    }

    pub fn set_username(&self, value: String) {
        self.ctx.borrow().ent_turn_username.set_text(value.as_str());
    }

    pub fn get_username(&self) -> String {
        self.ctx.borrow().ent_turn_username.get_text().to_string().clone()
    }

    pub fn set_password(&self, value: String) {
        self.ctx.borrow().ent_turn_password.set_text(value.as_str());
    }

    pub fn get_password(&self) -> String {
        self.ctx.borrow().ent_turn_password.get_text().to_string().clone()
    }

    pub fn get_srtp_keyring(&self) -> u32 {
        match self.ctx.borrow().cmb_turn_keyring.get_active() {
            Some(value) => value + 1,
            None => 0,
        }
    }

    pub fn set_srtp_keyring(&self, value: u32) {
        self.ctx.borrow().cmb_turn_keyring.set_active(Some(value -1));
    }

}


impl HelperFileSettings for SettingsTurnWidget {

    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        let use_turn = config.get("turn", "use_turn").unwrap();
        let use_tcp = config.get("turn", "use_tcp").unwrap();
        let use_rtcp_multiplexing = config.get("stun", "use_rtcp_multiplexing").unwrap();
        let server = config.get("stun", "server").unwrap();
        let port = config.get("stun", "port").unwrap();
        let username = config.get("stun", "username").unwrap();
        let password = config.get("stun", "password").unwrap();
        let srtp_keyring= config.get("stun", "srtp_keyring").unwrap();

        self.set_use_turn(use_turn.parse().unwrap());
        self.set_use_tcp(use_tcp.parse().unwrap());
        self.set_use_rtcp_multiplexing(use_rtcp_multiplexing.parse().unwrap());
        self.set_server(server);
        self.set_port(port.parse().unwrap());
        self.set_username(username);
        self.set_password(password);
        self.set_srtp_keyring(srtp_keyring.parse().unwrap());
    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        let use_turn = self.get_use_turn();
        let use_tcp = self.get_use_tcp();
        let use_rtcp_multiplexing = self.get_use_rtcp_multiplexing();
        let server = self.get_server();
        let port = self.get_port();
        let username = self.get_username();
        let password = self.get_password();
        let srtp_keyring = self.get_srtp_keyring();

        config.set("stun", "use_turn", Some(use_turn.to_string()));
        config.set("stun", "use_tcp", Some(use_tcp.to_string()));
        config.set("stun", "use_rtcp_multiplexing", Some(use_rtcp_multiplexing.to_string()));
        config.set("stun", "server", Some(server));
        config.set("stun", "port", Some(port.to_string()));
        config.set("stun", "username", Some(username));
        config.set("stun", "password", Some(password));
        config.set("stun", "srtp_keyring", Some(srtp_keyring.to_string()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}


