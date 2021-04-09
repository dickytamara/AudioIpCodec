

use gtk::prelude::*;

use gtk::{Label, Switch, ComboBoxText, SpinButton, Builder};
use glib::clone;

use std::cell::RefCell;


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

    pub fn set_ice_use_rtcp(&self, value: bool) {
        self.ctx.borrow().swt_ice_rtcp.set_state(value);
    }

    pub fn get_ice_use_rtcp(&self) -> bool {
        self.ctx.borrow().swt_ice_rtcp.get_state()
    }

}