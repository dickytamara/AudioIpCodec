
use gtk::prelude::*;
use gtk::{Builder, Notebook, Button};
use std::cell::RefCell;

use super::settings_call::SettingsCallWidget;
use super::settings_stun::SettingsStunWidget;
use super::settings_turn::SettingsTurnWidget;
use super::settings_ice::SettingsIceWidget;
use super::settings_proxy::SettingsProxyWidget;
use super::settings_dns::SettingsDnsWidget;

#[derive(Clone, Copy)]
pub enum SettingsCurrentActivePage {
    Call,
    Stun,
    Turn,
    Ice,
    Buffer,
    Media,
    Proxy,
    Dns
}


#[derive(Clone)]
pub struct SettingsWidgetStorage {
    // gtk notebook
    ntbk_settings: Notebook,
    btn_settings_reset: Button,
    btn_settings_save: Button,
    btn_settings_apply: Button,
}

impl SettingsWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsWidgetStorage {
            ntbk_settings: gtk_builder.get_object("ntbk_settings").unwrap(),
            btn_settings_reset: gtk_builder.get_object("btn_settings_reset").unwrap(),
            btn_settings_save: gtk_builder.get_object("btn_settings_save").unwrap(),
            btn_settings_apply: gtk_builder.get_object("btn_settings_apply").unwrap()
        }
    }
}

#[derive(Clone)]
pub struct SettingsWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsWidgetStorage>,
    pub call: SettingsCallWidget,
    pub stun: SettingsStunWidget,
    pub turn: SettingsTurnWidget,
    pub ice: SettingsIceWidget,
    pub proxy: SettingsProxyWidget,
    pub dns: SettingsDnsWidget,
}


impl SettingsWidget {
    pub fn new(gtk_builder: &gtk::Builder) -> Self {
        SettingsWidget {
            ctx: RefCell::new(SettingsWidgetStorage::new(gtk_builder)),
            call: SettingsCallWidget::new(gtk_builder),
            stun: SettingsStunWidget::new(gtk_builder),
            turn: SettingsTurnWidget::new(gtk_builder),
            ice: SettingsIceWidget::new(gtk_builder),
            proxy: SettingsProxyWidget::new(gtk_builder),
            dns: SettingsDnsWidget::new(gtk_builder),
        }
    }

    // get current active page
    pub fn get_current_active_page(&self) -> Option<SettingsCurrentActivePage> {
        match self.ctx.borrow().ntbk_settings.get_current_page() {
            Some(idx) => {
                match idx {
                    0 => Some(SettingsCurrentActivePage::Call),
                    1 => Some(SettingsCurrentActivePage::Stun),
                    2 => Some(SettingsCurrentActivePage::Turn),
                    3 => Some(SettingsCurrentActivePage::Ice),
                    4 => Some(SettingsCurrentActivePage::Buffer),
                    5 => Some(SettingsCurrentActivePage::Media),
                    6 => Some(SettingsCurrentActivePage::Proxy),
                    7 => Some(SettingsCurrentActivePage::Dns),
                    _ => None
                }
            },
            None => None
        }
    }

    pub fn reset_connect_clicked<F> (&self, callback: F)
    where
        F: Fn(Option<SettingsCurrentActivePage>) + 'static
    {
        let wid = self.clone();
        self.ctx.borrow()
        .btn_settings_reset.connect_clicked( move |_| {
            (callback)(wid .get_current_active_page());
        });
    }

    pub fn save_connect_clicked<F> (&self, callback: F)
    where
        F: Fn(Option<SettingsCurrentActivePage>) + 'static
    {
        let wid = self.clone();
        self.ctx.borrow()
        .btn_settings_save.connect_clicked( move |_| {
            (callback)(wid.get_current_active_page());
        });
    }

    pub fn apply_connect_clicked<F> (&self, callback: F)
    where
        F: Fn(Option<SettingsCurrentActivePage>) + 'static
    {
        let wid = self.clone();
        self.ctx.borrow()
        .btn_settings_apply.connect_clicked( move |_| {
            (callback)(wid.get_current_active_page());
        });
    }
}