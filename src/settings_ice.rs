

use gtk::prelude::*;

use gtk::{Label, Switch, ComboBoxText, SpinButton, Builder};
use std::cell::RefCell;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsIceWidgetStorage {
    // ice section
    lbl_use_ice: Label,
    lbl_ice_use_rtcp: Label,
    lbl_ice_reg_nomination: Label,
    lbl_ice_trickle_method: Label,
    lbl_ice_max_hosts: Label,
    swt_use_ice: Switch,
    swt_ice_rtcp: Switch,
    cmb_ice_reg_nomination: ComboBoxText,
    cmb_ice_trickle_method: ComboBoxText,
    spn_ice_max_hosts: SpinButton
}

impl SettingsIceWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsIceWidgetStorage {
            lbl_use_ice: gtk_builder.get_object("lbl_use_ice").unwrap(),
            lbl_ice_use_rtcp: gtk_builder.get_object("lbl_ice_use_rtcp").unwrap(),
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
pub struct SettingsIceWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsIceWidgetStorage>
}

impl SettingsIceWidget {

    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsIceWidget {
            ctx: RefCell::new(SettingsIceWidgetStorage::new(gtk_builder))
        };

        // set spin button ice max hosts sever
        result.ctx.borrow().spn_ice_max_hosts.set_digits(0);
        result.ctx.borrow().spn_ice_max_hosts.set_range(-1_f64, 256_f64);
        result.ctx.borrow().spn_ice_max_hosts.set_increments(1_f64, 5_f64);

        let result_clone = result.clone();
        result.ctx.borrow().swt_use_ice.connect_property_active_notify(move |s| {
            result_clone.set_use_ice(s.get_state());
        });

        result.set_use_ice(false);
        result
    }

    pub fn reset(&self) {
        // set default value for ice properties
        let context = self.ctx.borrow();
        context.swt_use_ice.set_state(false);
        context.swt_ice_rtcp.set_state(false);
        context.cmb_ice_reg_nomination.set_active_id(Some("Agresive"));
        context.cmb_ice_trickle_method.set_active_id(Some("Disabled"));
        context.spn_ice_max_hosts.set_value(-1_f64);
        self.set_use_ice(false);
    }

    pub fn set_use_ice(&self, value: bool) {
        let context = self.ctx.borrow();
        context.swt_ice_rtcp.set_sensitive(value);
        context.cmb_ice_reg_nomination.set_sensitive(value);
        context.cmb_ice_trickle_method.set_sensitive(value);
        context.spn_ice_max_hosts.set_sensitive(value);
    }

    pub fn get_use_ice(&self) -> bool {
        self.ctx.borrow().swt_use_ice.get_state()
    }

    pub fn set_use_rtcp(&self, value: bool) {
        self.ctx.borrow().swt_ice_rtcp.set_state(value);
    }

    pub fn get_use_rtcp(&self) -> bool {
        self.ctx.borrow().swt_ice_rtcp.get_state()
    }

    pub fn set_reg_nomination(&self, value: u32) {
        self.ctx.borrow().cmb_ice_reg_nomination.set_active(Some(value -1));
    }

    pub fn get_reg_nomination(&self) -> u32 {
        match self.ctx.borrow().cmb_ice_reg_nomination.get_active() {
            Some(value) => value +1,
            None => 0,
        }
    }

    pub fn set_trickle_method(&self, value: u32) {
        self.ctx.borrow().cmb_ice_trickle_method.set_active(Some(value -1));
    }

    pub fn get_trickle_method(&self) -> u32 {
        match self.ctx.borrow().cmb_ice_trickle_method.get_active() {
            Some(value) => value + 1,
            None => 0
        }
    }

    pub fn set_max_hosts(&self, value: f64) {
        self.ctx.borrow().spn_ice_max_hosts.set_value(value);
    }

    pub fn get_max_hosts(&self) -> f64 {
        self.ctx.borrow().spn_ice_max_hosts.get_value()
    }

}


impl HelperFileSettings for SettingsIceWidget {
    fn load(&self, path: &str) {
        let mut config = Ini::new();
        config.load(path).unwrap();

        let use_ice = config.get("ice", "use_ice").unwrap();
        let use_rtcp = config.get("ice", "use_srtp").unwrap();
        let reg_nomination = config.get("ice", "reg_nomination").unwrap();
        let trickle_method = config.get("ice", "trickle_method").unwrap();
        let max_hosts = config.get("ice", "max_hosts").unwrap();

        self.set_use_ice(use_ice.parse().unwrap());
        self.set_use_rtcp(use_rtcp.parse().unwrap());
        self.set_reg_nomination(reg_nomination.parse().unwrap());
        self.set_trickle_method(trickle_method.parse().unwrap());
        self.set_max_hosts(max_hosts.parse().unwrap());
    }

    fn save(&self, path: &str) {
        let mut config = Ini::new();
        config.load(path).unwrap();

        let use_ice = self.get_use_ice();
        let use_rtcp = self.get_use_rtcp();
        let reg_nomination = self.get_reg_nomination();
        let trickle_method = self.get_trickle_method();
        let max_hosts = self.get_max_hosts();

        config.set("ice", "use_ice", Some(use_ice.to_string()));
        config.set("ice", "use_rtcp", Some(use_rtcp.to_string()));
        config.set("ice", "reg_nomination", Some(reg_nomination.to_string()));
        config.set("ice", "trickle_method", Some(trickle_method.to_string()));
        config.set("ice", "max_hosts", Some(max_hosts.to_string()));

        config.write(path).unwrap();
    }
}