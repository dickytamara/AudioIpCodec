
use gtk::prelude::*;

use gtk::{Label, Button, SpinButton, ComboBoxText, Entry, Switch, Builder};
use glib::clone;

use std::cell::RefCell;


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
    btn_turn_save: Button,
    btn_turn_reset: Button,
}

impl SettingsTurnWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsTurnWidgetStorage {
            lbl_use_turn: gtk_builder.get_object("lbl_use_turn").unwrap(),
            lbl_turn_tcp: gtk_builder.get_object("lbl_turn_tcp").unwrap(),
            lbl_turn_rtcp_multiplexing: gtk_builder.get_object("lbl_turn_rtcp_mutiplexing").unwrap(),
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
        }
    }
}


pub struct SettingsTurnWidget {
    ctx: RefCell<SettingsTurnWidgetStorage>
}

impl SettingsTurnWidget {
    // inner data just borrow not mutate
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsTurnWidget {
            ctx: RefCell::new(SettingsTurnWidgetStorage::new(gtk_builder)),
        }
    }

    pub fn init(&self) {
        let context = self.ctx.borrow();
        // set spin button turn port server
        context.spn_turn_port.set_digits(0);
        context.spn_turn_port.set_range(3478_f64, 65535_f64);
        context.spn_turn_port.set_increments(1_f64, 5_f64);

    }

}





