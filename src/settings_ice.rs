

use gtk::prelude::*;

use gtk::{Label, Switch, ComboBoxText, SpinButton, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsIceWidgetStorage {
    // ice section
    lbl_use_ice: Label,
    lbl_use_rtcp: Label,
    lbl_aggressive: Label,
    lbl_trickle_method: Label,
    lbl_max_hosts: Label,
    swt_use_ice: Switch,
    swt_no_rtcp: Switch,
    swt_aggressive: Switch,
    cmb_trickle_method: ComboBoxText,
    spn_max_hosts: SpinButton
}

impl SettingsIceWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsIceWidgetStorage {
            lbl_use_ice: gtk_builder.get_object("lbl_use_ice").unwrap(),
            lbl_use_rtcp: gtk_builder.get_object("lbl_ice_no_rtcp").unwrap(),
            lbl_aggressive: gtk_builder.get_object("lbl_ice_aggressive").unwrap(),
            lbl_trickle_method: gtk_builder.get_object("lbl_ice_trickle_method").unwrap(),
            lbl_max_hosts: gtk_builder.get_object("lbl_ice_max_hosts").unwrap(),
            swt_use_ice: gtk_builder.get_object("swt_use_ice").unwrap(),
            swt_no_rtcp: gtk_builder.get_object("swt_ice_no_rtcp").unwrap(),
            swt_aggressive: gtk_builder.get_object("swt_ice_aggressive").unwrap(),
            cmb_trickle_method: gtk_builder.get_object("cmb_ice_trickle_method").unwrap(),
            spn_max_hosts: gtk_builder.get_object("spn_ice_max_hosts").unwrap()
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
        result.ctx.borrow().spn_max_hosts.set_digits(0);
        result.ctx.borrow().spn_max_hosts.set_range(-1_f64, 256_f64);
        result.ctx.borrow().spn_max_hosts.set_increments(1_f64, 5_f64);

        let result_clone = result.clone();
        result.ctx.borrow().swt_use_ice.connect_property_active_notify(move |s| {
            result_clone.set_use_ice(s.get_state());
        });

        result.set_use_ice(false);
        result
    }

    pub fn reset(&self) {
        // set default value for ice properties
        self.ctx.borrow().swt_use_ice.set_state(false);
        self.ctx.borrow().swt_no_rtcp.set_state(false);
        self.ctx.borrow().swt_aggressive.set_state(false);
        self.ctx.borrow().cmb_trickle_method.set_active_id(Some("Disabled"));
        self.ctx.borrow().spn_max_hosts.set_value(-1_f64);
        self.set_use_ice(false);
    }

    pub fn set_use_ice(&self, value: bool) {
        self.ctx.borrow().lbl_use_rtcp.set_sensitive(value);
        self.ctx.borrow().lbl_aggressive.set_sensitive(value);
        self.ctx.borrow().lbl_trickle_method.set_sensitive(value);
        self.ctx.borrow().lbl_max_hosts.set_sensitive(value);
        self.ctx.borrow().swt_no_rtcp.set_sensitive(value);
        self.ctx.borrow().swt_aggressive.set_sensitive(value);
        self.ctx.borrow().cmb_trickle_method.set_sensitive(value);
        self.ctx.borrow().spn_max_hosts.set_sensitive(value);
    }

    pub fn get_use_ice(&self) -> bool {
        self.ctx.borrow().swt_use_ice.get_state()
    }

    pub fn set_no_rtcp(&self, value: bool) {
        self.ctx.borrow().swt_no_rtcp.set_state(value);
    }

    pub fn get_no_rtcp(&self) -> bool {
        self.ctx.borrow().swt_no_rtcp.get_state()
    }

    pub fn set_aggressive(&self, value: bool) {
        self.ctx.borrow().swt_aggressive.set_state(value);
    }

    pub fn get_aggressive(&self) -> bool {
        self.ctx.borrow().swt_aggressive.get_state()
    }

    pub fn set_trickle_method(&self, value: u32) {
        self.ctx.borrow().cmb_trickle_method.set_active(Some(value -1));
    }

    pub fn get_trickle_method(&self) -> u32 {
        match self.ctx.borrow().cmb_trickle_method.get_active() {
            Some(value) => value + 1,
            None => 0
        }
    }

    pub fn set_max_hosts(&self, value: f64) {
        self.ctx.borrow().spn_max_hosts.set_value(value);
    }

    pub fn get_max_hosts(&self) -> f64 {
        self.ctx.borrow().spn_max_hosts.get_value()
    }

}


impl HelperFileSettings for SettingsIceWidget {
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        self.set_use_ice(config.get("ice", "use_ice").unwrap().parse().unwrap());
        self.set_no_rtcp(config.get("ice", "use_srtp").unwrap().parse().unwrap());
        self.set_aggressive(config.get("ice", "reg_nomination").unwrap().parse().unwrap());
        self.set_trickle_method(config.get("ice", "trickle_method").unwrap().parse().unwrap());
        self.set_max_hosts(config.get("ice", "max_hosts").unwrap().parse().unwrap());
    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        config.set("ice", "use_ice", Some(self.get_use_ice().to_string()));
        config.set("ice", "use_rtcp", Some(self.get_no_rtcp().to_string()));
        config.set("ice", "reg_nomination", Some(self.get_aggressive().to_string()));
        config.set("ice", "trickle_method", Some(self.get_trickle_method().to_string()));
        config.set("ice", "max_hosts", Some(self.get_max_hosts().to_string()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}